use tarpc::ServeHandle;
use server::*;
use std::time::{Duration, Instant};
use ticktock::timer::Timer;
use codec::Codec;
use log::LogLevel;
use env_logger;
use std::hash::{Hash, SipHasher, Hasher};

struct ClientPair {
    id: u64,
    client: Client,
}

struct TimerFlags {
    election: bool,
    leader: bool,
    discovery: bool,
    heartbeat: bool,
}

pub struct Node {
    serve_handle:    ServeHandle,
    clients:         Vec<ClientPair>,
    requests:        Vec<(Request, u32)>,
    id:              u64,
    addr:            Client,
    heartbeat_timer: Timer,
    discovery_timer: Timer,
    election_timer:  Timer,
    leader_timer:    Timer,
}

// Create unique id from host/peer string
pub fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = SipHasher::new();
    t.hash(&mut s);
    s.finish()
}

impl Node {
    pub fn new(host: String, timeout: u64) -> Self {
        Node {
            serve_handle: Server::new(hash(&host)).spawn(&host.as_str()).unwrap(),
            clients: Vec::new(),
            requests: Vec::new(),
            addr: Client::new(&host).unwrap(),
            id: hash(&host),
            // Create timers for election and new client discovery
            // Timer to track if election has timed out -  
            // If this times out during the election, the
            // election was unsuccessful and we need to
            // start a new election.
            election_timer: Timer::new(Duration::from_millis(timeout)),
            // Timer to track if leader has timed out
            leader_timer: Timer::new(Duration::from_millis(timeout)),
            // Timer to check for new clients
            discovery_timer: Timer::new(Duration::from_millis(1000)),
            // Timer to send heartbeat and log updates
            heartbeat_timer: Timer::new(Duration::from_millis(20)),
        }
    }

    pub fn start(&mut self, mut peers: Vec<String>) {
        // Start connections
        self.add_clients(&mut peers);

        // Initialize log
        env_logger::init().unwrap();
        info!("Log initialized");
        info!("Initial log value: {}", self.get_log_entry().1);

        // Operation loop
        loop {
            // Get current time
            let mut now = Instant::now();
            // Create timer flags
            let mut flags = TimerFlags {
                election: self.election_timer.has_fired(now),
                leader: self.leader_timer.has_fired(now),
                heartbeat: self.heartbeat_timer.has_fired(now),
                discovery: self.discovery_timer.has_fired(now)
            };

            // Send heartbeat and log updates
            if flags.heartbeat {
                self.tx_request();
                flags.heartbeat = false;
                self.heartbeat_timer.reset(now);
            }

            // Periodically check for new clients if
            // peers is not empty
            if flags.discovery && !peers.is_empty() {
                self.add_clients(&mut peers);
                flags.discovery = false;
                self.discovery_timer.reset(now);
            }

            // Check if leader is alive, reset timer if so
            if self.heartbeat_rcvd() {
                flags.leader = false;
                self.leader_timer.reset(now);
            }

            // Check for leader timeout
            if flags.leader {
                warn!("Leader has timed out: Initiating election");
                self.initiate_election();
                flags.leader = false;
                self.leader_timer.reset(now);
            }
        }
    }

    // TODO The logic in this function doesn't allow connections
    // to be made until all peers have been initialized, due to
    // the fact that the loop is broken on an Err(_) response
    // in the nested match statement
    fn add_clients(&mut self, peers: &mut Vec<String>) {
        let ref mut clients = self.clients;

        // Don't want to re-add existing clients
        while !peers.is_empty() {
            match peers.pop() {
                // New peer is present in peers
                Some(peer) =>
                    // Create new client
                    match Client::new(&peer) {
                        Ok(client)  => clients.push(ClientPair{ id: hash(&peer), client: client }),
                        Err(_) => {
                            // If creation is unsuccessful, push peer back onto
                            // peers and break loop. Wait for next discovery
                            // period to retry.
                            peers.push(peer);
                            break;
                        }
                    },
                // peers is empty
                None => break,
            }
        }
        info!("Connected to {} peers", clients.len());
    }

    // Send heartbeat and log instructions
    fn tx_request(&mut self) {
        let state_code = self.addr.get_state().unwrap();
        let clients = &self.clients;
        let mut requests = &mut self.requests;
        let state = Codec::decode_state(state_code);
        let mut writes = 0;
        let majority = (clients.len() + 1) / 2;

        if state != State::Leader {
            // If node is not leader, 
            // it doesn't need to send requests
            return;
        } else { 
            // Send request to non-leader nodes
            let (request, data) = match requests.pop() {
                Some(tuple) => tuple,
                None => (Request::Heartbeat, 0),
            };
            let op_code = Codec::encode_request(request);

            for client in clients {
                let reply = client.client.rx_request(op_code, data, self.id);
                // Handle reply
                match reply {
                    Ok(true) =>
                        match request {
                            Request::Commit    => continue,
                            Request::Heartbeat => continue,
                            // All write requests
                            _                  => writes += 1,
                        },
                     _ => {}// TODO Retransmit
                }
            }

            // Commit majority changes
            if writes > majority {
                // TODO commit change to own log
                let op_code = Codec::encode_request(Request::Commit);
                for client in clients {
                    client.client.rx_request(op_code, data, self.id);
                }
            }
        }
    }

    fn heartbeat_rcvd(&self) -> bool {
        self.addr.heartbeat_rcvd().unwrap()
    }

    fn initiate_election(&self) {
        // Initiate election
        let clients = &self.clients;
        // Node votes for itself
        let mut vote_count = 1;
        let majority = (clients.len() + 1) / 2;

        let state_code = Codec::encode_state(State::Candidate);
        self.addr.set_state(state_code);

        for client in clients {
            vote_count += match client.client.request_vote(self.id).unwrap() {
                true => 1,
                false => 0,
            };
            if vote_count > majority {
                info!("{} is the new leader", self.id);
                // New leader
                self.addr.set_leader();
            }
        }
    }

    fn get_log_entry(&self) -> (Request, i64) {
        let entry = self.addr.get_log_entry().unwrap();
        let request = Codec::decode_request(entry.0);
        let value = entry.1;
        (request, value)
    }

    pub fn drop_clients(&mut self) {
        let mut clients = &mut self.clients;
        while !clients.is_empty() {
            drop(clients.pop().unwrap().client);
        }
    }

    pub fn stop(self) {
        self.serve_handle.shutdown();
    }
}

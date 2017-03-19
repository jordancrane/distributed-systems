use tarpc::ServeHandle;
use server::*;
use std::time::{Duration, Instant};
use ticktock::timer::Timer;
use std::hash::{Hash, SipHasher, Hasher};

struct ClientPair {
    id: u64,
    client: Client,
}

pub struct Node {
    serve_handle: ServeHandle,
    clients: Vec<ClientPair>,
    id: u64,
    addr: Client,
    heartbeat_timer: Timer,
    election_timer: Timer,
}

// Create unique id from host/peer string
// Can't use strings since they don't implement
// the copy method
pub fn hash<T: Hash>(t: &T) -> u64 {
    let mut s = SipHasher::new();
    t.hash(&mut s);
    s.finish()
}

impl Node {
    pub fn new(host: String, timeout: u64) -> Self {
        Node {
            serve_handle: Server::new(hash(&host)).spawn(host.as_str()).unwrap(),
            clients: Vec::new(),
            addr: Client::new(host.clone()).unwrap(),
            id: hash(&host),
            election_timer: Timer::new(Duration::from_millis(timeout)),
            heartbeat_timer: Timer::new(Duration::from_millis(50)),
        }
    }

    pub fn start(&mut self, mut peers: Vec<String>) {
        // Periodically check for new clients if peers is not empty
        self.add_clients(&mut peers);
       
        // Reset timers
        self.election_timer.reset(Instant::now());
        self.heartbeat_timer.reset(Instant::now());

        loop {
            // Send heartbeat and log updates
            if self.heartbeat_timer.has_fired(Instant::now()) {
                self.broadcast_heartbeats();
                self.heartbeat_timer.reset(Instant::now());
            }

            // Check for leader timeout
            if self.election_timer.has_fired(Instant::now()) {
                if !self.heartbeat_rcvd() {
                    if self.drop_lost_leader() {
                        self.initiate_election();
                    }
                }
                self.election_timer.reset(Instant::now());
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
                        }
                    },
                    // peers is empty
                None => break,
            }
        }

        println!("Connected to {} peers", clients.len());
    }

    fn broadcast_heartbeats(&mut self) {
        let state = self.addr.get_state().unwrap();
        let clients = &self.clients;
        let term = self.addr.get_term().unwrap();

        if state != State::Leader {
            // If node is not leader, it doesn't need to send requests
            return;
        }

        for client in clients {
            client.client.heartbeat(self.id, term);
        }
    }

    fn heartbeat_rcvd(&self) -> bool {
        self.addr.get_heartbeat_rcvd().unwrap()
    }

    fn initiate_election(&mut self) {
        let clients = &self.clients;
        let state = self.addr.get_state().unwrap();

        if state != State::Leader {
            println!("Leader has timed out: Initiating election");
            self.addr.set_state(State::Candidate);
            // Initiate election
            // Node votes for itself
            let mut vote_count = 1;
            self.addr.set_voted_this_term();
            // Calculate majority
            let majority = (clients.len() + 1) / 2;

            for client in clients {
                vote_count += match client.client.request_vote(self.id) {
                    Ok(result) => {
                        match result {
                            true => 1,
                            false => 0,
                        }
                    }
                    Err(_) => 0,
                };
            }

            println!("Received {} vote(s) from {} node(s)", vote_count, clients.len() + 1);

            if vote_count > majority {
                self.addr.increment_term();
                println!("I am the new leader");
                self.addr.set_state(State::Leader);
            } else {
                println!("Election of {} failed", self.id);
                self.addr.set_state(State::Follower);
                self.addr.reset_voted_this_term();
            }
        }
    }

    fn drop_lost_leader(&mut self) -> bool {
        let leader_index = self.get_leader_index();
        let mut clients = &mut self.clients;

        match leader_index {
            Some(index) => 
                match clients.get(index).unwrap().client.is_alive() {
                    Ok(_) => { false },
                    Err(_) => {
                        println!("Dropping lost leader");
                        let lost_leader = clients.remove(index); 
                        drop(lost_leader.client);
                        true
                    }
                },
            None => { true }
        }

    }

    fn get_leader_index(&self) -> Option<usize> {
        let clients = &self.clients;
        for (index, client_pair) in clients.iter().enumerate() {
            if client_pair.id == self.addr.get_leader_id().unwrap() {
                return Some(index);
            }
        }
        None
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

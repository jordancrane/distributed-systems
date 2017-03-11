use tarpc::ServeHandle;
use server::*;
use std::time::{ Duration, Instant };
use ticktock::timer::Timer;
use codec::Codec;


pub struct Node {
    serve_handle:    ServeHandle,
    clients:         Vec<Client>,
    requests:        Vec<(Request, u32)>,
    addr:            Client,
    heartbeat_timer: Timer,
    discovery_timer: Timer,
    election_timer:  Timer,
    leader_timer:    Timer,
}

impl Node {
    pub fn new(host: String, timeout: u64) -> Self {

        Node {
            serve_handle: Server::new().spawn(&host.as_str()).unwrap(),
            clients: Vec::new(),
            requests: Vec::new(),
            addr: Client::new(host).unwrap(),
            // Create timers for election and new client discovery
            election_timer: Timer::new(Duration::from_millis(timeout)),
            leader_timer: Timer::new(Duration::from_millis(timeout)),
            discovery_timer: Timer::new(Duration::from_millis(1000)),
            heartbeat_timer: Timer::new(Duration::from_millis(50)),
        }
    }

    pub fn start(&mut self, mut peers: Vec<String>) {

        // Start connections
        self.add_clients(&mut peers);

        // Operation loop
        loop {
            // Get current time
            let now = Instant::now();

            // Send heartbeat and log updates
            if self.heartbeat_timer.has_fired(now){
                self.tx_request();
                self.heartbeat_timer.reset(now);
            }

            // Periodically check for new clients
            if self.discovery_timer.has_fired(now) {
                self.add_clients(&mut peers);
                self.discovery_timer.reset(now);
            }

            // Check if leader is alive, reset timer if so
            if self.heartbeat_rcvd() {
                self.election_timer.reset(now);
            }

            // Check for leader timeout
            if self.leader_timer.has_fired(now) {
                self.initiate_election();
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
                        Ok(c)  => clients.push(c),
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
        println!("Connected to {} peers", clients.len());
    }

    // Send heartbeat and log instructions
    fn tx_request(&mut self) {
        let state_code = self.addr.get_state().unwrap();
        let clients = &self.clients;
        let mut requests = &mut self.requests;
        let state = Codec::decode_state(state_code);

        // If node is not leader, it doesn't need to send requests
        if state != State::Leader {
            return;
        }
        // Send request to non-leader nodes
        else {
            let (request, data) = requests.pop().unwrap();
            let op_code = Codec::encode_request(request);
            for client in clients {
                let reply = client.rx_request(op_code, data);
                // TODO Handle reply
            }
        }
    }

    pub fn drop_clients(&mut self) {
        let mut clients = &mut self.clients;
        while !clients.is_empty() {
            drop(clients.pop());
        }
    }

    fn heartbeat_rcvd(&self) -> bool {
        self.addr.heartbeat_rcvd().unwrap()
    }

    fn initiate_election(&self) {
        // TODO Initiate election
    }

    pub fn stop(self) {
        self.serve_handle.shutdown();
    }
}

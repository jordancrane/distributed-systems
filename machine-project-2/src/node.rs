use tarpc::ServeHandle;
use server::*;
use std::time::{ Duration, Instant };
use ticktock::timer::Timer;

pub struct Node {
    pub serve_handle: ServeHandle,
    pub clients:      Vec<Client>,
    addr:  Client,
    heartbeat_timer: Timer,
    discovery_timer: Timer,
    election_timer: Timer,
}

impl Node {
    pub fn new(host: String, timeout: u64) -> Self {

        Node {
            serve_handle: Server::new().spawn(&host.as_str()).unwrap(),
            clients: Vec::new(),
            addr: Client::new(host).unwrap(),
            // Create timers for election and new client discovery
            election_timer: Timer::new(Duration::from_millis(timeout)),
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
                self.send_message();
                self.heartbeat_timer.reset(now);
            }

            // Periodically check for new clients
            if self.discovery_timer.has_fired(now) {
                self.add_clients(&mut peers);
                self.discovery_timer.reset(now);
            }

            // Check if leader is alive, reset timer if so
            if self.is_leader_alive() {
                self.election_timer.reset(now);
            }

            // Check for election timeout
            if self.election_timer.has_fired(now) {
                self.initiate_election();
                self.election_timer.reset(now);
            }
        }
    }

    // TODO The logic in this function doesn't allow connections
    // to be made until all peers have been initialized, due to
    // the fact that the loop is broken on an Err(_) response
    // in the nested match statement
    pub fn add_clients(&mut self, peers: &mut Vec<String>) {
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

    pub fn drop_clients(&self) {
        let clients = &self.clients;
        for client in clients {
            drop(client);
        }
    }

    pub fn is_leader_alive(&self) -> bool {
        // TODO Check for heartbeat reciept
        // Return true for now
        true
    }

    pub fn initiate_election(&self) {
        // TODO Initiate election
    }

    pub fn send_message(&self) {
        // TODO send message and log updates
    }

    // This is a function to test inter-server communication
    pub fn notify(&self) {
        let s = self.addr.report_term().unwrap();
        println!("Term: {}", s);
    }
}

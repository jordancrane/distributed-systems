use tarpc::ServeHandle;
use server::*;
use std::time::{Duration, Instant};
use ticktock::timer::Timer;

struct ClientPair {
    id: String,
    client: Client,
}

pub struct Node {
    serve_handle: ServeHandle,
    clients: Vec<ClientPair>,
    id: String,
    addr: Client,
    heartbeat_timer: Timer,
    election_timer: Timer,
}

impl Node {
    pub fn new(host: String, timeout: u64) -> Self {
        Node {
            serve_handle: Server::new(host.clone()).spawn(host.as_str()).unwrap(),
            clients: Vec::new(),
            addr: Client::new(host.clone()).unwrap(),
            id: host,
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

            // Check if leader is alive, reset timer if so
            if self.heartbeat_rcvd() {
                self.election_timer.reset(Instant::now());
            }

            // Check for leader timeout
            if self.election_timer.has_fired(Instant::now()) {
                self.initiate_election();
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
                        Ok(client)  => clients.push(ClientPair{ id: peer, client: client }),
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

        if state != State::Leader {
            // If node is not leader, it doesn't need to send requests
            return;
        }

        println!("Sending heartbeat!");
        for client in clients {
            client.client.heartbeat(self.id.clone());
        }
    }

    fn heartbeat_rcvd(&self) -> bool {
        self.addr.get_heartbeat_rcvd().unwrap()
    }

    fn initiate_election(&mut self) {
        self.election_timer.reset(Instant::now());
        let state = self.addr.get_state().unwrap();

        if state != State::Leader {
            println!("Leader has timed out: Initiating election");
            self.addr.set_state(State::Candidate);

            // Initiate election
            let clients = &self.clients;
            // Node votes for itself
            let mut vote_count = 1;
            let majority = (clients.len() + 1) / 2;

            // TODO identify server to clients
            for client in clients {
                vote_count += match client.client.request_vote(self.id.clone()) {
                    Ok(result) => {
                        match result {
                            true => 1,
                            false => 0,
                        }
                    }
                    Err(_) => 0,
                };
            }

            println!("Received {} votes", vote_count);

            if vote_count > majority {
                println!("{} is the new leader", self.id);
                self.addr.set_state(State::Leader);
            } else {
                self.addr.set_state(State::Follower);
            }
        }
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

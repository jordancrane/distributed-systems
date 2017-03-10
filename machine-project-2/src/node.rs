use tarpc::ServeHandle;
use server::*;

pub struct Node {
    pub serve_handle: ServeHandle,
    pub clients:      Vec<Client>,
    addr:  Client,
}

impl Node {
    pub fn new(host: String) -> Self {
        Node {
            serve_handle: Server::new().spawn(&host.as_str()).unwrap(),
            clients: Vec::new(),
            addr: Client::new(host).unwrap(),
        }
    }

    pub fn start(&self) {
        // TODO Move main loop here
    }

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

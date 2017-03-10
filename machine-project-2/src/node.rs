use std::sync::RwLock;
use std::sync::Arc;
use tarpc::ServeHandle;
use std::sync::mpsc::{self, Receiver, Sender};

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    Leader,
    Candidate,
    Follower,
}

service! {
    rpc request_vote();
    rpc vote();
    rpc notify() -> String;
}

#[derive(Clone)]
pub struct Server {
    state: Arc<RwLock<State>>,
    term: Arc<RwLock<usize>>,
    vote_count: Arc<RwLock<usize>>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            state: Arc::new(RwLock::new(State::Follower)),
            term: Arc::new(RwLock::new(0)),
            vote_count: Arc::new(RwLock::new(0)),

        }
    }
}

impl Service for Server {
    fn request_vote(&self) {
        let mut state = self.state.write().unwrap();

        if *state == State::Follower {
            // vote yes
        }

        if *state == State::Candidate {
            // vote no
        }
    }

    fn vote(&self) {
        let mut state = self.state.write().unwrap();
        let mut vote_count = self.vote_count.write().unwrap();

        if *state == State::Candidate {
            *vote_count += 1;

            // Does this node contain a majority?
            if *vote_count > 2 {
                *state = State::Leader;
            }
        }
    }

    fn notify(&self) -> String {
        "notify recieved".to_string()
    }
}

pub struct Node {
    pub serve_handle: ServeHandle,
    clients:      Vec<Client>,
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

    pub fn add_clients(&mut self, peers: Vec<String>) {
        let ref mut clients = self.clients;
        for peer in peers {
            // TODO Need retry logic since we can't start all servers at
            // exactly the same time
            println!("Creating client for {}", peer);
            match Client::new(peer) {
                Ok(c)  => clients.push(c),
                Err(_) => {},
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

    pub fn notify(&self) {
        let s = self.addr.notify().unwrap();
        println!("{}", &s);
    }
}

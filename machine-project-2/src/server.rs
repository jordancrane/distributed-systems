use std::sync::RwLock;
use std::sync::Arc;
use std::io;
use codec::Codec;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    Leader,
    Candidate,
    Follower,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Request {
    Add,
    Sub,
    Set,
    Commit,
}

service! {
    rpc request_vote();
    rpc vote();
    rpc rx_request(operation: u8, data: u32) -> bool;
    rpc get_state() -> u8;
    rpc heartbeat_rcvd() -> bool;
}

#[derive(Clone)]
pub struct Server {
    state: Arc<RwLock<State>>,
    term: Arc<RwLock<usize>>,
    vote_count: Arc<RwLock<usize>>,
    heartbeat_rcvd: Arc<RwLock<bool>>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            state: Arc::new(RwLock::new(State::Follower)),
            term: Arc::new(RwLock::new(0)),
            vote_count: Arc::new(RwLock::new(0)),
            heartbeat_rcvd: Arc::new(RwLock::new(false)),
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

    fn heartbeat_rcvd(&self) -> bool {
        let mut heartbeat_rcvd = self.heartbeat_rcvd.write().unwrap();
        let state = self.state.read().unwrap();

        // Only check for heartbeat if follower
        match *state {
            State::Follower => 
                // Check if heartbeat has been received
                match *heartbeat_rcvd {
                    true  => {
                        // If so, unset flag and return true (this will
                        // reset the timer)
                        *heartbeat_rcvd = false;
                        true
                    },
                    // Else return false. This will not initiate an 
                    // election, it just will not reset the timer
                    false => false
                },
            State::Leader | State::Candidate => false
        }
    }

    fn rx_request(&self, op_code: u8, data: u32) -> bool {
        // Heartbeat recieved
        let mut heartbeat_rcvd = self.heartbeat_rcvd.write().unwrap();
        *heartbeat_rcvd = true;

        let request = Codec::decode_request(op_code);
        // TODO Handle log request

        true
    }

    fn get_state(&self) -> u8 {
        let state = self.state.read().unwrap();
        Codec::encode_state(*state)
    }
}

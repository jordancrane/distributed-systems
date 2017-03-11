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
    Heartbeat
}

service! {
    rpc request_vote() -> bool;
    rpc vote();
    rpc rx_request(operation: u8, data: u32) -> bool;
    rpc get_state() -> u8;
    rpc heartbeat_rcvd() -> bool;
    rpc get_log_entry() -> (u8, i64);
    rpc get_term() -> usize;
}

#[derive(Clone)]
pub struct Server {
    state: Arc<RwLock<State>>,
    term: Arc<RwLock<usize>>,
    vote_count: Arc<RwLock<usize>>,
    heartbeat_rcvd: Arc<RwLock<bool>>,
    log_staging: Arc<RwLock<Vec<(u8, i64)>>>,
    log: Arc<RwLock<Vec<(u8, i64)>>>,
    voted_this_term: Arc<RwLock<bool>>,
}

impl Server {
    pub fn new() -> Self {
        Server {
            state: Arc::new(RwLock::new(State::Follower)),
            term: Arc::new(RwLock::new(0)),
            vote_count: Arc::new(RwLock::new(0)),
            heartbeat_rcvd: Arc::new(RwLock::new(false)),
            log_staging: Arc::new(RwLock::new(Vec::new())),
            log: Arc::new(RwLock::new(vec![(0, 0)])),
            voted_this_term: Arc::new(RwLock::new(false)),
        }
    }

    fn append_log(&self, request: Request, data: u32) -> bool {
        true
    }
    
    fn commit_log(&self) -> bool {
        true
    }
}

impl Service for Server {
    fn request_vote(&self) -> bool {
        let state = self.state.read().unwrap();
        let mut voted_this_term = self.voted_this_term.write().unwrap();

        if *state == State::Follower && !*voted_this_term {
            // vote yes
            *voted_this_term = true;
            true
        } else {
            // vote no
            false
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
                    // Else return false (this will not initiate an 
                    // election, it just will not reset the timer)
                    false => false
                },
            // If leader or candidate, we do not check
            // for the heartbeat, as we are either sending
            // the heartbeat, or an election is in progress
            State::Leader | State::Candidate => false
        }
    }

    fn rx_request(&self, op_code: u8, data: u32) -> bool {
        // Heartbeat recieved
        let mut heartbeat_rcvd = self.heartbeat_rcvd.write().unwrap();
        *heartbeat_rcvd = true;

        let request = Codec::decode_request(op_code);
        // Handle log request
        match request {
            Request::Set => { self.append_log(request, data) },
            Request::Add => { self.append_log(request, data) },
            Request::Sub => { self.append_log(request, data) },
            Request::Commit => { self.commit_log() },
            Request::Heartbeat => { true },
        }
    }

    fn get_log_entry(&self) -> (u8, i64) {
        let log = self.log.read().unwrap();
        *log.last().unwrap()
    }

    fn get_state(&self) -> u8 {
        let state = self.state.read().unwrap();
        Codec::encode_state(*state)
    }

    fn get_term(&self) -> usize {
        *self.term.read().unwrap()
    }
}

use std::sync::RwLock;
use std::sync::Arc;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum State {
    Leader,
    Candidate,
    Follower,
}

service! {
    rpc request_vote(client_id: String) -> bool;
    rpc heartbeat(client_id: String);
    rpc get_state() -> State;
    rpc get_term() -> usize;
    rpc set_state(state: State);
    rpc get_heartbeat_rcvd() -> bool;
}

#[derive(Clone)]
pub struct Server {
    state: Arc<RwLock<State>>,
    term: Arc<RwLock<usize>>,
    vote_count: Arc<RwLock<usize>>,
    heartbeat_rcvd: Arc<RwLock<bool>>,
    voted_this_term: Arc<RwLock<bool>>,
    leader_id: Arc<RwLock<String>>,
    id: Arc<RwLock<String>>,
}

impl Server {
    pub fn new(id: String) -> Self {
        Server {
            state: Arc::new(RwLock::new(State::Follower)),
            term: Arc::new(RwLock::new(0)),
            vote_count: Arc::new(RwLock::new(0)),
            heartbeat_rcvd: Arc::new(RwLock::new(false)),
            voted_this_term: Arc::new(RwLock::new(false)),
            leader_id: Arc::new(RwLock::new("No Leader".to_string())),
            id: Arc::new(RwLock::new(id)),
        }
    }
}

impl Service for Server {
    fn request_vote(&self, client_id: String) -> bool {
        let state = self.state.read().unwrap();
        let mut voted_this_term = self.voted_this_term.write().unwrap();
        let mut leader_id = self.leader_id.write().unwrap();

        if *state == State::Follower && !*voted_this_term {
            // vote yes
            *voted_this_term = true;
            *leader_id = client_id.clone();
            println!("Voted for {}", client_id);
            true
        } else {
            // vote no
            false
        }
    }

    fn get_heartbeat_rcvd(&self) -> bool {
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
                    false => { false }
                },
            // If leader or candidate, we do not check
            // for the heartbeat, as we are either sending
            // the heartbeat, or an election is in progress
            State::Leader => { true },
            State::Candidate => { false },
        }
    }

    fn heartbeat(&self, client_id: String) {
        println!("Received heartbeat");

        let mut state = self.state.write().unwrap();
        let mut leader_id = self.leader_id.write().unwrap();
        let mut heartbeat_rcvd = self.heartbeat_rcvd.write().unwrap();

        *leader_id = client_id;
        *heartbeat_rcvd = true;
        *state = State::Follower;
    }

    fn get_state(&self) -> State {
        *self.state.read().unwrap()
    }

    fn get_term(&self) -> usize {
        *self.term.read().unwrap()
    }

    fn set_state(&self, state: State) {
        *self.state.write().unwrap() = state;

        if state == State::Leader {
            let mut leader_id = self.leader_id.write().unwrap();
            let id = self.id.read().unwrap().clone();

            *leader_id = id;
        }
    }
}

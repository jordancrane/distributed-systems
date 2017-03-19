use std::sync::Arc;
use atomic::{Atomic, Ordering};
use node::hash;

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum State {
    Leader,
    Candidate,
    Follower,
}

service! {
    rpc request_vote(client_id: u64) -> bool;
    rpc heartbeat(client_id: u64);
    rpc get_state() -> State;
    rpc get_term() -> usize;
    rpc set_state(state: State);
    rpc get_heartbeat_rcvd() -> bool;
}

#[derive(Clone)]
pub struct Server {
    state: Arc<Atomic<State>>,
    term: Arc<Atomic<usize>>,
    vote_count: Arc<Atomic<usize>>,
    heartbeat_rcvd: Arc<Atomic<bool>>,
    voted_this_term: Arc<Atomic<bool>>,
    leader_id: Arc<Atomic<u64>>,
    id: u64,
}

impl Server {
    pub fn new(id: u64) -> Self {
        Server {
            state: Arc::new(Atomic::new(State::Follower)),
            term: Arc::new(Atomic::new(0)),
            vote_count: Arc::new(Atomic::new(0)),
            heartbeat_rcvd: Arc::new(Atomic::new(false)),
            voted_this_term: Arc::new(Atomic::new(false)),
            leader_id: Arc::new(Atomic::new(hash(&"No Leader".to_string()))),
            id: id,
        }
    }
}

impl Service for Server {
    fn request_vote(&self, client_id: u64) -> bool {
        if self.state.load(Ordering::Relaxed) == State::Follower && !self.voted_this_term.load(Ordering::Relaxed) {
            // vote yes
            self.voted_this_term.store(true, Ordering::Relaxed);
            self.leader_id.store(client_id, Ordering::Relaxed);
            println!("Voted for {}", client_id);
            true
        } else {
            // vote no
            false
        }
    }

    fn get_heartbeat_rcvd(&self) -> bool {
        // Only check for heartbeat if follower
        match self.state.load(Ordering::Relaxed) {
            State::Follower =>
                // Check if heartbeat has been received
                match self.heartbeat_rcvd.load(Ordering::Relaxed) {
                    true  => {
                        // If so, unset flag and return true (this will
                        // reset the timer)
                        self.heartbeat_rcvd.store(false, Ordering::Relaxed);
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

    fn heartbeat(&self, client_id: u64) {
        println!("Received heartbeat");

        self.leader_id.store(client_id, Ordering::Relaxed);
        self.heartbeat_rcvd.store(true, Ordering::Relaxed);
        self.state.store(State::Follower, Ordering::Relaxed);
    }

    fn get_state(&self) -> State {
        self.state.load(Ordering::Relaxed)
    }

    fn get_term(&self) -> usize {
        self.term.load(Ordering::Relaxed)
    }

    fn set_state(&self, state: State) {
        self.state.store(state, Ordering::Relaxed);

        if state == State::Leader {
            self.leader_id.store(self.id, Ordering::Relaxed);
        }
    }
}

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
    rpc get_leader_id() -> u64;
    rpc get_heartbeat_rcvd() -> bool;
    rpc get_state() -> State;
    rpc get_term() -> usize;
    rpc heartbeat(client_id: u64, term: usize);
    rpc increment_term();
    rpc is_alive() -> bool;
    rpc reset_voted_this_term();
    rpc request_vote(client_id: u64) -> bool;
    rpc set_state(state: State);
    rpc set_voted_this_term();
}

#[derive(Clone)]
pub struct Server {
    election_result_pending: Arc<Atomic<bool>>,
    heartbeat_rcvd: Arc<Atomic<bool>>,
    id: u64,
    leader_id: Arc<Atomic<u64>>,
    state: Arc<Atomic<State>>,
    term: Arc<Atomic<usize>>,
    vote_count: Arc<Atomic<usize>>,
    voted_this_term: Arc<Atomic<bool>>,
}

impl Server {
    pub fn new(id: u64) -> Self {
        println!("I am {}", id);
        Server {
            election_result_pending: Arc::new(Atomic::new(false)),
            heartbeat_rcvd: Arc::new(Atomic::new(false)),
            id: id,
            leader_id: Arc::new(Atomic::new(0)),
            state: Arc::new(Atomic::new(State::Follower)),
            term: Arc::new(Atomic::new(0)),
            vote_count: Arc::new(Atomic::new(0)),
            voted_this_term: Arc::new(Atomic::new(false)),
        }
    }
}

impl Service for Server {
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

    fn get_leader_id(&self) -> u64 {
        self.leader_id.load(Ordering::Relaxed)
    }

    fn get_state(&self) -> State {
        self.state.load(Ordering::Relaxed)
    }

    fn get_term(&self) -> usize {
        self.term.load(Ordering::Relaxed)
    }

    fn heartbeat(&self, client_id: u64, term: usize) {
        if self.election_result_pending
            .load(Ordering::Relaxed) {
            self.leader_id.store(client_id, Ordering::Relaxed);
            self.election_result_pending.store(false, Ordering::Relaxed);
            println!("Term: {}", term);
        }
        self.term.store(term, Ordering::Relaxed);
        self.heartbeat_rcvd.store(true, Ordering::Relaxed);
        self.state.store(State::Follower, Ordering::Relaxed);
    }

    fn increment_term(&self) {
        let term = self.term.load(Ordering::Relaxed) + 1;
        println!("Term: {}", term);
        self.term.store(term, Ordering::Relaxed);
    }

    fn is_alive(&self) -> bool {
        true
    }

    fn reset_voted_this_term(&self) {
        self.voted_this_term.store(false, Ordering::Relaxed);
    }

    fn set_voted_this_term(&self) {
        self.voted_this_term.store(true, Ordering::Relaxed);
    }

    fn request_vote(&self, candidate_id: u64) -> bool {
        self.election_result_pending.store(true, Ordering::Relaxed);
        if self.state.load(Ordering::Relaxed) == State::Follower && !self.voted_this_term.load(Ordering::Relaxed) {
            // increment term
            self.voted_this_term.store(true, Ordering::Relaxed);
            // set new leader id
            self.leader_id.store(candidate_id, Ordering::Relaxed);
            println!("Voted for {}", candidate_id);
            // vote yes
            true
        } else {
            // vote no
            false
        }
    }

    fn set_state(&self, state: State) {
        self.state.store(state, Ordering::Relaxed);

        if state == State::Leader {
            self.leader_id.store(self.id, Ordering::Relaxed);
        }
    }
}

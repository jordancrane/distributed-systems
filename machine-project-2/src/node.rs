use std::sync::RwLock;
use std::sync::Arc;

#[derive(Copy, Clone, Debug, PartialEq)]
enum State {
    Leader,
    Candidate,
    Follower,
}

service! {
    rpc request_vote();
    rpc vote();
}

#[derive(Clone)]
pub struct Server {
    state: Arc<RwLock<State>>,
    term: Arc<RwLock<usize>>,
    vote_count: Arc<RwLock<usize>>,
}

impl Server {
    pub fn new() -> Server {
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
}

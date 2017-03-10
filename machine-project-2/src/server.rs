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
    rpc increment_term();
    rpc report_term() -> usize;
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

    // This is a function to test inter-server communication
    fn increment_term(&self) {
        let mut term = self.term.write().unwrap();
        *term += 1;
    }

    // This is a function to test inter-server communication
    fn report_term(&self) -> usize {
        *self.term.read().unwrap()
    }

    // This is a function to test inter-server communication
    fn notify(&self) -> String {
        "notify recieved".to_string()
    }
}

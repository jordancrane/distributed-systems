#[derive(Copy, Clone, Debug)]
enum State {
    Leader,
    Candidate,
    Follower,
}

service! {
    rpc request_vote();
    rpc vote();
    rpc append_entries();
}

#[derive(Clone)]
pub struct Server {
    state: State,
    term: usize,
    vote_count: usize,
}

impl Server {
    pub fn new() -> Server {
        Server { 
            state: State::Follower, 
            term: 0,
            vote_count: 0,
        }
    }

    fn log(&self) {
        println!("State: {:?}", self.state);
    }
}

impl Service for Server {
    fn request_vote(&self) {
    }

    fn vote(&self) {
    }

    fn append_entries(&self) {
    }
}

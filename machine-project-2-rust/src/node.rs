#[derive(Copy, Clone, Debug)]
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
        if self.state == State::Follower {
            // vote yes 
        }
    
        if self.state == State::Candidate {
            // vote no
        }
    }

    fn vote(&self) {
        if self.state == State::Candidate {
            self.vote_count += 1;
           
            // Does this node contain a majority?
            // if self.vote_count > ( / 2) {
            //   self.state = State::Leader;
            // }
        }
    }
}

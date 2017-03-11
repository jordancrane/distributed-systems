// Simple codec to allow serialized transmission in the background
// while still permitting the use of helpful enums
pub mod Codec {
    use server::State;
    use server::Request;

    // Can't selialize enums over tarpc, so need to encode the 
    // integer state code
    pub fn encode_state(state: State) -> u8 {
        match state {
            State::Leader    => return 0,
            State::Candidate => return 1,
            State::Follower  => return 2,
        }
    }

    // Can't selialize enums over tarpc, so need to decode the 
    // integer state code
    pub fn decode_state(state_code: u8) -> State {
        match state_code {
            0 => return State::Leader,
            1 => return State::Candidate,
            2 => return State::Follower,
            _ => panic!("No State"),
        }
    }

    // Can't selialize enums over tarpc, so need to encode the 
    // integer operation code
    pub fn encode_request(request: Request) -> u8 {
        match request {
            Request::Set    => return 0,
            Request::Add    => return 1,
            Request::Sub    => return 2,
            Request::Commit => return 3,
        }
    }

    // Can't selialize enums over tarpc, so need to decode the 
    // integer operation code
    pub fn decode_request(op_code: u8) -> Request {
        match op_code {
            0 => return Request::Set,
            1 => return Request::Add,
            2 => return Request::Sub,
            3 => return Request::Commit,
            _ => panic!("No such op code"),
        }
    }
}

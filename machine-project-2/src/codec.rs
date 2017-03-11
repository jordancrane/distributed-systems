// Simple codec to allow serialized transmission in the background
// while still permitting the use of helpful enums
pub mod Codec {
    use server::State;
    use server::Request;

    // Can't selialize enums over tarpc, so need to encode the 
    // integer state code
    pub fn encode_state(state: State) -> u8 {
        match state {
            State::Leader    => { 0 },
            State::Candidate => { 1 },
            State::Follower  => { 2 },
        }
    }

    // Can't selialize enums over tarpc, so need to decode the 
    // integer state code
    pub fn decode_state(state_code: u8) -> State {
        match state_code {
            0 => { State::Leader },
            1 => { State::Candidate },
            2 => { State::Follower },
            _ => panic!("No State"),
        }
    }

    // Can't selialize enums over tarpc, so need to encode the 
    // integer operation code
    pub fn encode_request(request: Request) -> u8 {
        match request {
            Request::Set    => { 0 },
            Request::Add    => { 1 },
            Request::Sub    => { 2 },
            Request::Commit => { 3 },
            Request::Heartbeat => { 4 },
        }
    }

    // Can't selialize enums over tarpc, so need to decode the 
    // integer operation code
    pub fn decode_request(op_code: u8) -> Request {
        match op_code {
            0 => { Request::Set },
            1 => { Request::Add },
            2 => { Request::Sub },
            3 => { Request::Commit },
            4 => { Request::Heartbeat },
            _ => panic!("No such op code"),
        }
    }
}

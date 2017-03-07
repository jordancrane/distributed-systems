use rand;
use std::thread;
use std::time::Duration;

enum State {
    Leader,
    Candidate,
    Follower,
}

enum Message {
    RequestVote,
    VoteYes,
    VoteNo,
    IAmTheLeader 
}

/// "1/100 chance of crashing"
fn did_crash() -> bool {
    let i = rand::random::<usize>();
    return i % 100 == 0; 
}

const NUMBER_OF_NODES: usize = 100;

fn log(node_id: usize, message: &'static str) {
    println!("[Node {node_id:>0width$}] {message}", 
        node_id=node_id, width=2, message=message);
}

fn run_node(node_id: usize) {
    loop {
        if did_crash() {
            log(node_id, "Crashed!");
            break;
        }

        log(node_id, "Working...");

        thread::sleep(Duration::from_millis(50));
    }
}

// let mut threads = Vec::new();
// 
// for i in 0..NUMBER_OF_NODES {
//     let thread = thread::spawn(move || run_node(i));
//     threads.push(thread);
// }
// 
// for thread in threads {
//     thread.join();
// }


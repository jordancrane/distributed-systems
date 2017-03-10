#[macro_use]
extern crate tarpc;
extern crate rand;
extern crate clap;
extern crate ticktock;

mod config;
mod node;

use node::*;
use std::thread::sleep;
use std::time::{ Duration, Instant };
use ticktock::timer::Timer;
use rand::Rng;

fn main() {
    // Fetch config
    let (host, mut peers) = config::fetch_cli_options();

    // Make random between 150ms and 300ms
    let timeout = rand::thread_rng().gen_range(150, 301);

    // Create timers for election and new client discovery
    let mut election_timer = Timer::new(Duration::from_millis(timeout));
    let mut discovery_timer = Timer::new(Duration::from_millis(1000));

    // Start server
    println!("Creating server on {}", host);
    let mut node = Node::new(host);

    // Start connections
    node.add_clients(&mut peers);

    // Leader timeout loop
    loop {
        // Get current time
        let now = Instant::now();
        
        // Periodically check for new clients
        if discovery_timer.has_fired(now) {
            node.add_clients(&mut peers);
            discovery_timer.reset(now);
        }

        // Check for election timeout
        if election_timer.has_fired(now) {
            // TODO initiate election
            // [Start test code]
            for client in &node.clients {
                client.increment_term();
            }
            node.notify();
            // [End test code]
            election_timer.reset(now);
        }
    }

    // Stop connections
    node.drop_clients();

    // Stop server
    node.serve_handle.shutdown();
}

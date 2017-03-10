#[macro_use]
extern crate tarpc;
extern crate rand;
extern crate clap;
extern crate ticktock;

mod config;
mod server;
mod node;

use node::*;
use server::*;
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
    let mut heartbeat = Timer::new(Duration::from_millis(50));

    // Start server
    println!("Creating server on {}", host);
    let mut node = Node::new(host);

    // Start connections
    node.add_clients(&mut peers);

    // Leader timeout loop
    loop {
        // Get current time
        let now = Instant::now();

        // Send heartbeat and log updates
        if heartbeat.has_fired(now){
            node.send_message();
            heartbeat.reset(now);
        }

        // Periodically check for new clients
        if discovery_timer.has_fired(now) {
            node.add_clients(&mut peers);
            discovery_timer.reset(now);
        }

        // Check if leader is alive, reset timer if so
        if node.is_leader_alive() {
            election_timer.reset(now);
        }

        // Check for election timeout
        if election_timer.has_fired(now) {
            node.initiate_election();
            election_timer.reset(now);
        }
    }

    // Stop connections
    node.drop_clients();

    // Stop server
    node.serve_handle.shutdown();
}

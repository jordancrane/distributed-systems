#[macro_use]
extern crate tarpc;
extern crate rand;
extern crate clap;

mod config;
mod node;

use node::*;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Fetch config
    let (host, peers) = config::fetch_cli_options();

    // TODO Make random between 150ms and 300ms
    let timeout = Duration::from_millis(150);

    // Start server
    println!("Creating server on {}", host);
    let mut node = Node::new(host);

    // Start connections
    node.add_clients(peers);

    // Leader timeout loop
    loop {
        node.notify();
        sleep(timeout);
    }

    // Stop connections
    node.drop_clients();

    // Stop server
    node.serve_handle.shutdown();
}

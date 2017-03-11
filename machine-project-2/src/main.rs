#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate tarpc;
extern crate rand;
extern crate clap;
extern crate ticktock;

mod config;
mod server;
mod node;
mod codec;

use node::*;
use server::*;
use rand::Rng;

fn main() {
    // Fetch config
    let (host, peers) = config::fetch_cli_options();

    // Make random timeout between 150ms and 300ms
    let timeout = rand::thread_rng().gen_range(150, 301);

    // Start server
    println!("Creating server on {}", host);
    let mut node = Node::new(host, timeout);

    // Leader timeout loop
    node.start(peers);

    // Stop connections
    node.drop_clients();

    // Stop server
    node.stop();
}

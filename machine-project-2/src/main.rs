#[macro_use]
extern crate tarpc;
#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate rand;
extern crate ticktock;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod config;
mod server;
mod node;

use node::*;
use rand::Rng;

fn main() {
    // Fetch config
    let (host, peers) = config::fetch_cli_options();

    // Make random timeout between 150ms and 300ms
    // let timeout = rand::thread_rng().gen_range(150, 301);
    let timeout = rand::thread_rng().gen_range(300, 600);
    println!("{}", timeout);

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

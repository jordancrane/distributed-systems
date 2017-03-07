#[macro_use] extern crate tarpc;
extern crate rand;
extern crate clap;

mod config;
mod node;

use node::*;
use config::{fetch_cli_options};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    // Fetch config
    let (host, peers) = fetch_cli_options(); 
    let mut clients: Vec<Client> = Vec::new();

    // Start server
    println!("Creating server on {}", host);
    let server = Server::new().spawn(host.as_str()).unwrap();

    // Start connections
    for peer in peers {
        // TODO Need retry logic since we can't start all servers at 
        // exactly the same time
        println!("Creating client for {}", peer);
        let client = Client::new(peer);
        match client {
            Ok(c) => clients.push(c),
            Err(..) => {},
        }
    }

    println!("Connected to {} peers", clients.len());

    // Event loop
    loop {
        println!("Tick");

        // Election timeout (between 150ms to 300ms)
        // client.request_vote();

        sleep(Duration::from_millis(100));
    }

    // Stop connections
    for client in clients {
        drop(client);
    }

    // Stop server
    server.shutdown();
}


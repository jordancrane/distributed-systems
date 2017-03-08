#[macro_use] extern crate tarpc;
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
    let mut clients = Vec::new();

    // TODO Make random between 150ms and 300ms    
    let timeout = Duration::from_millis(150); 

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

    // Leader timeout loop
    loop {
        sleep(timeout);
    }

    // Stop connections
    for client in clients {
        drop(client);
    }

    // Stop server
    server.shutdown();
}


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
    let (host, peers) = fetch_cli_options(); 
    println!("Host:\t{}", host);
    println!("Peers:\t{:?}", peers);

    let addr = "127.0.0.1:9000";
    let server = Server::new().spawn(addr).unwrap();

    /*
       for peer in peers {
       println!("Connecting to {}", peer);

       let client = Client::new(peer).unwrap();
       println!("{}", client.hello("Jonah".to_string()).unwrap());
       drop(client);
       }
       */

    // Election timeout (between 150ms to 300ms)

    loop {
        println!("Hello, world!");
        sleep(Duration::from_millis(10));
    }

    // let client = Client::new(addr).unwrap();
    // client.request_vote();
    // drop(client);

    server.shutdown();
}


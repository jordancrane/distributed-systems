#[macro_use] extern crate tarpc;
#[macro_use] extern crate serde_derive;
extern crate serde_json;
extern crate rand;
extern crate clap;

mod config;
mod node;
mod rpc;

use rpc::*;
use config::{fetch_cli_options};

fn main() {
    let (host, peers) = fetch_cli_options(); 
    println!("Host:\t{}", host);
    println!("Peers:\t{:?}", peers);

    let addr = "127.0.0.1:9000";
    let server = Server.spawn(addr).unwrap();

    /*
    for peer in peers {
        println!("Connecting to {}", peer);

        let client = Client::new(peer).unwrap();
        println!("{}", client.hello("Jonah".to_string()).unwrap());
        drop(client);
    }
    */

    let client = Client::new(addr).unwrap();
    println!("{}", client.hello("Jonah".to_string()).unwrap());
    drop(client);
    
    server.shutdown();
}


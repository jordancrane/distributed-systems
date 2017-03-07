use serde_json;
use clap::{App, Arg};

#[derive(Deserialize)]
pub struct Config {
    pub nodes: Vec<Node>,
}

#[derive(Deserialize)]
pub struct Node {
    pub name: String,
        pub host: String,
}

/*
pub fn fetch_config() -> Config {
    let config: Config = serde_json::from_str(r#"
            {
            "nodes": [
            { "name": "Mercury", "host": "0.0.0.0:5050" },
            { "name": "Venus",   "host": "0.0.0.0:5051" },
            { "name": "Earth",   "host": "0.0.0.0:5052" }
            ]
            }
            "#).unwrap();

            config 
}
*/

pub fn fetch_cli_options() -> (String, Vec<String>) {
    let matches = App::new("Raft Leader Election Demo")
        .version("0.0.1")
        .author("Jonah G. <jonah@george@me.com>")
        .arg(Arg::with_name("host")
                .required(true)
                .long("host")
                .takes_value(true))
        .arg(Arg::with_name("peer")
                .long("peer")
                .multiple(true)
                .takes_value(true))
        .get_matches();

    let host = matches.value_of("host").unwrap();
    let peers: Vec<String> = matches.values_of("peer").unwrap()
        .map(|s| s.to_owned())
        .collect();

    (host.to_string(), peers)
}


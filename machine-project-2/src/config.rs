use clap::{App, Arg};

pub fn fetch_cli_options() -> (String, Vec<String>) {
    let matches = App::new("Raft Leader Election Demo")
        .version("0.0.1")
        .author(crate_authors!())
        .arg(Arg::with_name("host").required(true).long("host").takes_value(true))
        .arg(Arg::with_name("peer").long("peer").multiple(true).takes_value(true))
        .get_matches();

    let host = matches.value_of("host").unwrap();
    let peers: Vec<String> = matches.values_of("peer")
        .unwrap()
        .map(|s| s.to_owned())
        .collect();

    (host.to_string(), peers)
}

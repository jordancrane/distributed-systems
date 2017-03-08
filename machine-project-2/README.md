# Raft Leader Election

## Setup

```sh
# Install Rustup
curl https://sh.rustup.rs -sSf | sh

# Set stable Rust to default
rustup default stable

# Confirm it works
rustc --version
```

## Running

```sh
cargo run -- --host localhost:5000 --peer localhost:5001 
```

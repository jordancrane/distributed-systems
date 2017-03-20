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
#Terminal 1:
cargo run -- --host localhost:5000 --peer localhost:5001 --peer localhost:5002 --peer localhost:5003

#Terminal 2:
cargo run -- --host localhost:5001 --peer localhost:5000 --peer localhost:5002 --peer localhost:5003

#Terminal 3:
cargo run -- --host localhost:5002 --peer localhost:5000 --peer localhost:5001 --peer localhost:5003

#Terminal 4:
cargo run -- --host localhost:5003 --peer localhost:5000 --peer localhost:5001 --peer localhost:5002
```

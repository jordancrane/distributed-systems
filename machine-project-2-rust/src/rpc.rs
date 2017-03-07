service! {
    rpc hello(name: String) -> String;
}

#[derive(Clone)]
pub struct Server;

impl Service for Server {
    fn hello(&self, s: String) -> String {
        format!("Hello, {}!", s)
    }
}

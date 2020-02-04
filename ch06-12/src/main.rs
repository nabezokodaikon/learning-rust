mod client {
    pub fn echo() {
        println!("Client");
    }
}

mod network;
use network::server;

fn main() {
    server::echo();
    network::server::echo();
    client::echo();
    network::ping();
}

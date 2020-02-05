mod network;
use network::client;
use network::server;

fn main() {
    network::ping();
    client::echo();
    server::echo();
}

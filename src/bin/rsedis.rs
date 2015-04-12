extern crate rsedis;

use rsedis::networking::Server;

fn main() {
    let port: i32 = 6379;
    let server = Server::new("127.0.0.1", &port);
    server.run();
}
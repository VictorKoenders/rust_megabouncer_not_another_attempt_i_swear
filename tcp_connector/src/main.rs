#![cfg_attr(debug_assertions, allow(warnings))]

#[macro_use(try_get_field)]
extern crate shared;

use shared::*;

mod tcp_connector;
mod tcp_connection;

pub use tcp_connector::TcpConnector;
pub use tcp_connection::TcpConnection;

fn main() {
    let mut client = Client::new("tcp connector");
    client.register(TcpState::default(), vec![Box::new(TcpConnector {})]);
    client.run();
}

#[derive(Default)]
struct TcpState {
    pub connections: Vec<TcpConnection>,
}



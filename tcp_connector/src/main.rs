#![cfg_attr(debug_assertions, allow(warnings))]

#[macro_use(try_get_field)]
extern crate shared;
extern crate mio;

use shared::Client;

mod structs;
mod connector;
mod data;

pub use structs::{TcpState, ConnectionState, TcpConnection};
pub use connector::TcpConnector;
pub use data::TcpData;

fn main() {
    let mut client = Client::new("tcp connector");
    client.register(TcpState::default(), vec![Box::new(TcpConnector {})]);
    client.run();
}

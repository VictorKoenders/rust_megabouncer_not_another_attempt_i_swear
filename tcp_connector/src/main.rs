#![cfg_attr(debug_assertions, allow(warnings))]

#[macro_use(try_get_field)]
extern crate shared;

use shared::listener::client::*;
use shared::listener::traits::*;

fn main() {
    let mut client = Client::new("tcp connector");
    client.register(TcpState::default(), vec![Box::new(TcpConnector {})]);
    client.run();
}

#[derive(Default)]
struct TcpState {
    pub connections: Vec<TcpConnection>,
}

pub enum ConnectionState {
    Connected,
    Disconnected,
}

struct TcpConnection {
    pub host: String,
    pub port: i32,
    pub state: ConnectionState,
}

impl TcpConnection {
    pub fn new(host: String, port: i32) -> TcpConnection {
        TcpConnection {
            host: host,
            port: port,
            state: ConnectionState::Disconnected,
        }
    }
    pub fn connect() {}
    pub fn disconnect() {}
}

struct TcpConnector {}

impl Listener<TcpState> for TcpConnector {
    fn channels(&self) -> Vec<&str> {
        vec!["tcp.connect", "tcp.disconnect", "tcp.status"]
    }
    fn handle(&mut self, state: &mut TcpState, request: &mut Request) -> Result<()> {
        let host = try_get_field!(request, host, as_string);
        let port = try_get_field!(request, port, as_i32);
        let tcp_connector = match state.connections.iter().position(|c| {
            &c.host == host && &c.port == port
        }) {
            Some(index) => &mut state.connections[index],
            None => {
                state.connections.push(TcpConnection::new(
                    host.clone(),
                    port.clone(),
                ));
                state.connections.last_mut().unwrap()
            }
        };
        match request.channel.to_string().as_str() {
            "tcp.connect" => {}
            "tcp.disconnect" => {}
            "tcp.status" => {}
            x => unreachable!(),
        }
        Ok(())
    }
}

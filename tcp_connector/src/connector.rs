use super::{TcpConnection, TcpState, ConnectionState};
use shared::{Listener, Message, Request, Result, Value};
use std::collections::HashMap;

pub struct TcpConnector {}

impl Listener<TcpState> for TcpConnector {
    fn channels(&self) -> Vec<&str> {
        vec!["tcp.connect", "tcp.disconnect", "tcp.get.status"]
    }
    fn handle(&mut self, state: &mut TcpState, request: &mut Request) -> Result<()> {
        let tcp_connector = state.find_connection(request)?;
        match request.channel.to_string().as_str() {
            "tcp.connect" => {
                if tcp_connector.state == ConnectionState::Disconnected {
                    tcp_connector.connect();
                }
            }
            "tcp.disconnect" => {
                if tcp_connector.state == ConnectionState::Connected {
                    tcp_connector.disconnect();
                }
            }
            "tcp.get.status" => {}
            x => unreachable!(),
        }
        request.send(Message::Emit(String::from("tcp.status"), {
            let mut map = HashMap::new();
            map.insert(
                String::from("host"),
                Value::String(tcp_connector.host.clone()),
            );
            map.insert(String::from("port"), Value::I32(tcp_connector.port));
            map.insert(
                String::from("status"),
                Value::String(tcp_connector.state.to_string()),
            );
            map
        }));
        Ok(())
    }
}

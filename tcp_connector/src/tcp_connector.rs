use super::{TcpState, TcpConnection};
use shared::{Listener, Request, Result};

pub struct TcpConnector {}

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

use shared::{Result, Request};
use mio::net::TcpStream;
use mio::Token;

#[derive(Default)]
pub struct TcpState {
    pub connections: Vec<TcpConnection>,
}

impl TcpState {
    pub fn find_connection(&mut self, request: &mut Request) -> Result<&mut TcpConnection> {
        let host = try_get_field!(request, host, as_string);
        let port = try_get_field!(request, port, as_i32);
        Ok(match self.connections.iter().position(|c| {
            &c.host == host && &c.port == port
        }) {
            Some(index) => &mut self.connections[index],
            None => {
                self.connections.push(
                    TcpConnection::new(host.clone(), *port),
                );
                let connector = self.connections.last_mut().unwrap();
                let token = request.register(connector.stream.as_ref().unwrap())?;
                connector.token = token;
                connector
            }
        })
    }
}

#[derive(PartialEq)]
pub enum ConnectionState {
    Connected,
    Disconnected,
}

impl ToString for ConnectionState {
    fn to_string(&self) -> String {
        match *self {
            ConnectionState::Connected => String::from("connected"),
            ConnectionState::Disconnected => String::from("disconnected"),
        }
    }
}

pub struct TcpConnection {
    pub host: String,
    pub port: i32,
    pub state: ConnectionState,
    pub stream: Option<TcpStream>,
    pub token: Token,
}

impl TcpConnection {
    pub fn new(host: String, port: i32) -> TcpConnection {
        TcpConnection {
            host: host,
            port: port,
            state: ConnectionState::Disconnected,
            stream: None,
            token: Token(0),
        }
    }
    pub fn connect(&mut self) {}
    pub fn disconnect(&mut self) {}
}

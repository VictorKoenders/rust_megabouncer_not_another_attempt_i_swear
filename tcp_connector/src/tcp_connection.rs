pub enum ConnectionState {
    Connected,
    Disconnected,
}

pub struct TcpConnection {
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



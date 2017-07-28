use mio::{Evented, Token, Poll, PollOpt, Ready};
use std::io::{Read, Result, Write};
use mio::net::TcpStream;
use std::net::IpAddr;

pub struct Connector {
    connection: TcpStream,
    token: Token,
}

impl Connector {
    pub fn new(ip: IpAddr, port: u16) -> Result<Connector> {
        let connector = Connector {
            connection: TcpStream::connect(&(ip, port).into())?,
            token: Token(1),
        };
        Ok(connector)
    }

    pub fn register(&self, poll: &Poll) -> Result<()> {
        self.connection.register(
            poll,
            self.token,
            Ready::readable() | Ready::writable(),
            PollOpt::edge(),
        )?;
        Ok(())
    }

    pub fn token(&self) -> &Token {
        &self.token
    }
}

impl Write for Connector {
    fn write(&mut self, buff: &[u8]) -> Result<usize> {
        self.connection.write(buff)
    }

    fn flush(&mut self) -> Result<()> {
        self.connection.flush()
    }
}

impl Read for Connector {
    fn read(&mut self, buffer: &mut [u8]) -> Result<usize> {
        self.connection.read(buffer)
    }
}

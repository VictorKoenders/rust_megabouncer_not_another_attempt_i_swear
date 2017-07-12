use mio::tcp::TcpStream;
use std::net::SocketAddr;
use mio::Token;
use super::ClientEvent;
use shared::tcp_listener::TcpReader;
use shared::listener::traits::{Message, Value};
use super::channel::Channel;
use std::collections::HashMap;

pub struct Client {
    reader: TcpReader,
    pub listeners: Vec<Channel>,
    addr: SocketAddr,
    token: Token,
    pub name: Option<String>,
}

impl Client {
    pub fn new(stream: TcpStream, addr: SocketAddr, token: Token) -> Client {
        Client {
            reader: TcpReader::new(stream),
            listeners: Vec::new(),
            addr: addr,
            token: token,
            name: None,
        }
    }


    pub fn token(&self) -> Token {
        self.token
    }
    pub fn connected(&self) -> bool {
        self.reader.connected
    }

    pub fn is_listening_to(&self, chnl: &Channel) -> bool {
        for channel in &self.listeners {
            if channel.matches(chnl) {
                return true;
            }
        }
        false
    }

    pub fn emit(&mut self, message: Message) {
        self.reader.write(message);
        self.reader.process_write_queue();
    }

    pub fn update(&mut self, event: &mut ClientEvent) {
        if event.event.readiness().is_readable() {
            for message in self.reader.read() {
                if self.name.is_none() && !message.is_identify() {
                    self.emit(Message::no_name_error());
                    continue;
                }
                match message {
                    Message::Emit(str, map) => {
                        event.broadcast(str, map);
                    }
                    Message::Identify(name) => {
                        if self.name.is_none() {
                            self.name = Some(name.clone());
                            event.broadcast_identified(name);
                        } else {
                            self.emit(Message::already_has_name_error())
                        }
                    }
                    Message::RegisterListeners(listeners) => {
                        for listener in &listeners {
                            self.listeners.push(Channel::from(listener));
                        }
                        event.broadcast("client.registered.listeners", {
                            let mut map = HashMap::new();
                            map.insert(
                                String::from("client"),
                                Value::String(self.name.clone().unwrap()),
                            );
                            map.insert(
                                String::from("listeners"),
                                Value::Array(listeners.into_iter().map(Value::String).collect()),
                            );
                            map
                        });
                    }
                    /*x => {
                        println!("Got a weird message from client {:?}", x);
                    }*/
                }
            }
        }
        if event.event.readiness().is_writable() {
            self.reader.process_write_queue();
        }
    }
}

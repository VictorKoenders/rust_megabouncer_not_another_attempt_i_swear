use super::{Connector, SendMessage, ReceiveMessage};
use std::io::{ErrorKind, Read, Result, Write};
use std::collections::{HashMap, VecDeque};
use shared::{Message, Listener};
use mio::{Event, Token, Poll, Ready};
use std::net::IpAddr;

pub struct Redis {
    connector: Connector,
    write_queue: VecDeque<SendMessage>,
    read_queue: VecDeque<ReceiveMessage>,
    requested_keys: VecDeque<String>,
    buffer: Vec<u8>,
    writable: bool,
    readable: bool,
}
impl Default for Redis {
    fn default() -> Redis {
        Redis::connect([127u8, 0u8, 0u8, 1u8].into(), 6379).unwrap()
    }
}
impl Redis {
    pub fn connect(host: IpAddr, port: u16) -> Result<Redis> {
        let redis = Redis {
            connector: Connector::new(host, port)?,
            write_queue: Default::default(),
            read_queue: Default::default(),
            requested_keys: VecDeque::new(),
            buffer: Vec::new(),
            writable: false,
            readable: false,
        };
        Ok(redis)
    }

    pub fn token(&self) -> &Token {
        self.connector.token()
    }

    pub fn send(&mut self, send: SendMessage) -> Result<()> {
        self.write_queue.push_back(send);
        if self.writable {
            self.try_write()?;
        }
        Ok(())
    }

    fn try_write(&mut self) -> Result<()> {
        self.writable = false;
        while let Some(message) = self.write_queue.pop_front() {
            let vec: Vec<u8> = message.into();
            self.connector.write_all(&vec)?;
        }
        self.writable = true;
        Ok(())
    }

    fn try_read(&mut self) -> Result<()> {
        self.readable = false;
        let mut buffer = [0u8; 1024];
        let mut did_read = false;
        loop {
            match self.connector.read(&mut buffer) {
                Err(ref e) if e.kind() == ErrorKind::WouldBlock && did_read => {
                    self.process_read_queue();
                    return Ok(());
                }
                Err(e) => return Err(e),
                Ok(0) => {
                    self.process_read_queue();
                    return Ok(());
                }
                Ok(n) => {
                    did_read = true;
                    self.buffer.extend(&buffer[..n]);
                }
            }
        }
    }

    fn process_read_line(&mut self, offset: usize) -> Option<String> {
        let index = match self.buffer.iter().position(|n| n == &b'\n') {
            None => return None,
            Some(i) => i,
        };
        debug_assert_eq!(self.buffer[index - 1], b'\r');
        let line: Vec<_> = self.buffer
            .drain(..index + offset)
            .skip(offset)
            .take(index - (1 + offset))
            .collect();
        Some(String::from_utf8(line).unwrap())
    }

    fn process_read_queue(&mut self) {
        loop {
            if self.buffer.is_empty() {
                return;
            }
            println!("{:?}", ::std::str::from_utf8(&self.buffer));
            match self.buffer[0] {
                b'+' => {
                    // String, read to end
                    if let Some(str) = self.process_read_line(1) {
                        self.read_queue.push_back(ReceiveMessage::String(str));
                    }
                }
                b'-' => {
                    // Error, read to end
                    if let Some(str) = self.process_read_line(1) {
                        self.read_queue.push_back(ReceiveMessage::Error(str));
                    }
                }
                b':' => {
                    // Integer, i64,read a string and parse it
                    if let Some(str) = self.process_read_line(1) {
                        match str.parse() {
                            Ok(n) => self.read_queue.push_back(ReceiveMessage::Integer(n)),
                            Err(e) => {
                                println!("Could not parse i64 {:?}", str);
                                println!("{:?}", e);
                            }
                        }
                    }
                }
                b'$' => {
                    // Bulk string
                    // The first line is $n where n is the number of lines to follow
                    // The next n bytes (after the \r\n) are the actual string
                    // The 2 bytes after that should be \r\n
                    let index = match self.buffer.iter().position(|n| n == &b'\n') {
                        None => return,
                        Some(i) => i,
                    };
                    debug_assert_eq!(self.buffer[index - 1], b'\r');
                    if self.buffer[1] == b'-' {
                        debug_assert_eq!(self.buffer[2], b'1');
                        self.read_queue.push_back(ReceiveMessage::Null);
                        self.buffer.drain(..index + 1);
                    } else {
                        let mut n = 0usize;
                        for c in &self.buffer[1..index - 1] {
                            if c < &b'0' || c > &b'9' {
                                panic!("Expected number, got {:?}", self.buffer);
                            }
                            n = n * 10 + (c - b'0') as usize;
                        }

                        if self.buffer.len() <= index + n + 2 {
                            return;
                        }
                        let str = self.buffer
                            .drain(..index + n + 3)
                            .skip(index + 1)
                            .take(n)
                            .collect::<Vec<_>>();
                        if let Ok(str) = String::from_utf8(str) {
                            self.read_queue.push_back(ReceiveMessage::String(str));
                        }
                    }
                }
                x => {
                    panic!("Error: {:?} not implemented", x as char);
                }
            }
        }
    }
}

pub struct TState;
impl Listener<TState> for Redis {
    fn channels(&self) -> Vec<&str> {
        vec!["data.get", "data.set"]
    }

    fn register_poll(&mut self, poll: &Poll) -> ::shared::Result<Vec<Token>> {
        self.connector.register(poll)?;
        Ok(vec![*self.connector.token()])
    }

    fn handle(
        &mut self,
        _state: &mut TState,
        request: &mut ::shared::Request,
    ) -> ::shared::Result<()> {
        if request.channel.is("data.get") {
            if let Some(key) = request.data.get("key").and_then(|v| v.as_string()) {
                self.requested_keys.push_back(key.clone());
                self.send(SendMessage::Get { name: key.clone() }).unwrap();
            } else {
                request.send(::shared::Message::missing_key_error("key"));
            }
        } else if request.channel.is("data.set") {
            if let Some(key) = request.data.get("key").and_then(|v| v.as_string()) {
                if let Some(value) = request.data.get("value").and_then(|v| v.as_string()) {
                    self.requested_keys.push_back(key.clone());
                    self.send(SendMessage::Set {
                        name: key.clone(),
                        value: value.clone(),
                    }).unwrap();
                    self.send(SendMessage::Get { name: key.clone() }).unwrap();
                } else {
                    request.send(::shared::Message::missing_key_error("value"));
                }
            } else {
                request.send(::shared::Message::missing_key_error("value"));
            }
        }
        Ok(())
    }

    fn handle_event(&mut self, event: &Event, messages: &mut Vec<Message>) -> ::shared::Result<()> {
        if event.readiness().contains(Ready::writable()) {
            self.try_write()?;
        }
        if event.readiness().contains(Ready::readable()) {
            self.try_read()?;

            while !self.read_queue.is_empty() && !self.requested_keys.is_empty() {
                let item = self.read_queue.pop_front().unwrap();
                let key = self.requested_keys.pop_front().unwrap();

                messages.push(Message::Emit(format!("data.gotten.{}", key), {
                    let mut map = HashMap::new();
                    map.insert(String::from("value"), item.into_value());
                    map
                }));
            }

            println!("Messages: {:?}", messages);
        }
        Ok(())
    }
}

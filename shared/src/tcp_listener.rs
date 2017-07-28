use mio::net::TcpStream;
use super::traits::{Message, Result};
use serde_json::from_str;
use std::io::{Read, Write};

pub struct TcpReader {
    stream: TcpStream,
    read_buffer: Vec<u8>,
    write_buffer: Vec<u8>,
    writable: bool,
    pub connected: bool,
}

impl TcpReader {
    pub fn new(stream: TcpStream) -> TcpReader {
        TcpReader {
            stream: stream,
            read_buffer: Vec::with_capacity(1024),
            write_buffer: Vec::with_capacity(1024),
            writable: false,
            connected: true,
        }
    }

    pub fn read(&mut self) -> Vec<Message> {
        let mut messages = Vec::new();

        let mut buffer = [0u8; 1024];
        let mut first = true;
        'mainLoop: loop {
            match self.stream.read(&mut buffer) {
                Err(ref e) if e.kind() == ::std::io::ErrorKind::WouldBlock && !first => {
                    break;
                }
                Ok(0) if first => {
                    println!("Socket hung up");
                    self.connected = false;
                    break;
                }

                Ok(length) => {
                    first = false;
                    self.read_buffer.extend_from_slice(&buffer[0..length]);

                    while let Some(index) = self.read_buffer.iter().position(|c| c == &b'\n') {
                        let drain = self.read_buffer.drain(..index + 1).collect::<Vec<_>>();
                        match ::std::str::from_utf8(&drain[..drain.len() - 1]) {
                            Ok(str) => {
                                match from_str(str) {
                                    Ok(value) => {
                                        messages.push(value);
                                    }
                                    Err(e) => {
                                        println!("Could not parse {:?}", str);
                                        println!("{:?}", e);
                                        break 'mainLoop;
                                    }
                                }
                            }
                            Err(e) => {
                                println!("Could not parse string: {:?}", e);
                                break 'mainLoop;
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("Could not read: {} ({:?})", e, e.kind());
                    break;
                }
            };
        }

        messages
    }

    pub fn write(&mut self, message: Message) {
        let mut str = ::serde_json::to_string(&message).unwrap();
        str += "\n";
        let bytes = str.as_bytes();
        self.write_buffer.extend(bytes);
        if self.writable {
            self.process_write_queue().unwrap();
        }
    }

    pub fn process_write_queue(&mut self) -> Result<()> {
        match self.stream.write(&self.write_buffer) {
            Ok(length) => {
                self.write_buffer.drain(..length);
                self.writable = self.write_buffer.is_empty();
                Ok(())
            }
            Err(e) => {
                println!("Could not write: {:?}", e);
                self.connected = false;
                self.writable = false;
                Err(super::traits::ListenerError::Unknown(
                    format!("Could not write {:?}", e),
                ))
            }
        }
    }
}

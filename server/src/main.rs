#![cfg_attr(debug_assertions, allow(warnings))]

extern crate shared;
extern crate serde;
extern crate serde_json;
extern crate mio;

mod client;
mod server;
#[cfg(test)]
mod tests;

use mio::{Token, Events, Event, Poll, Ready, PollOpt};
use std::collections::HashMap;
use shared::{Message, Value};
use mio::net::TcpListener;
use std::net::Ipv4Addr;
use client::Client;


fn main() {
    let mut server = server::Server::default();

    server.run();
}

pub struct ClientEvent<'a> {
    pub event: &'a Event,
    pub broadcasts: Vec<(String, HashMap<String, Value>)>,
    pub did_identify: bool,
}

impl<'a> ClientEvent<'a> {
    pub fn broadcast<T: ToString>(&mut self, str: T, map: HashMap<String, Value>) {
        self.broadcasts.push((str.to_string(), map))
    }

    pub fn broadcast_identified(&mut self, name: String) {
        self.broadcasts.push((String::from("client.identified"), {
            let mut map = HashMap::new();
            map.insert(String::from("name"), Value::String(name));
            map
        }));
        self.did_identify = true;
    }
}

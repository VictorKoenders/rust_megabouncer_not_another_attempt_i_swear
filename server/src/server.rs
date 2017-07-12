use super::client::Client;
use mio::{Token, Poll, Events, Event, Ready, PollOpt};
use std::net::Ipv4Addr;
use mio::net::TcpListener;
use std::collections::HashMap;
use shared::listener::traits::{Message, Value};

#[derive(Default)]
pub struct Server {
    clients: Vec<Client>,
}

const SERVER_TOKEN: Token = Token(0);


impl Server {
    pub fn run(mut self) {
        let mut listener = TcpListener::bind(&(Ipv4Addr::new(127, 0, 0, 1), 12345).into()).unwrap();

        let mut poll = Poll::new().unwrap();
        poll.register(&listener, SERVER_TOKEN, Ready::readable(), PollOpt::edge())
            .unwrap();
        let mut next_token = 1;


        let mut events = Events::with_capacity(1024);
        loop {
            poll.poll(&mut events, None).unwrap();
            for event in events.iter() {
                let mut remove_index = None;
                if event.token() == SERVER_TOKEN {
                    let (client, addr) = listener.accept().unwrap();
                    poll.register(
                        &client,
                        Token(next_token),
                        Ready::readable() | Ready::writable(),
                        PollOpt::edge(),
                    ).unwrap();
                    self.clients.push(
                        Client::new(client, addr, Token(next_token)),
                    );
                    next_token += 1;
                    println!("got client, {:?}", addr);
                } else if let Some(index) = self.clients.iter().position(
                    |c| c.token() == event.token(),
                )
                {
                    let (broadcasts, did_identify, remove) = {
                        let client = &mut self.clients[index];

                        let mut event = super::ClientEvent {
                            event: &event,
                            broadcasts: Vec::new(),
                            did_identify: false,
                        };
                        client.update(&mut event);

                        (event.broadcasts, event.did_identify, !client.connected())
                    };

                    if remove {
                        self.clients.swap_remove(index);
                    } else if did_identify {
                        let (before, client, after) = {
                            let slice = &mut self.clients;
                            let (first_half, second_half) = slice.split_at_mut(index);
                            let (item, second_half) = second_half.split_first_mut().unwrap();
                            (first_half, item, second_half)
                        };
                        for item in before.iter().chain(after.iter()).filter(
                            |i| i.name.is_some(),
                        )
                        {
                            client.emit(Message::Emit(String::from("client.identified"), {
                                let mut map = HashMap::new();
                                map.insert(
                                    String::from("name"),
                                    Value::String(item.name.clone().unwrap()),
                                );
                                map
                            }));
                            client.emit(Message::Emit(
                                String::from("client.registered.listeners"),
                                {
                                    let mut map = HashMap::new();
                                    map.insert(
                                        String::from("client"),
                                        Value::String(item.name.clone().unwrap()),
                                    );
                                    map.insert(
                                        String::from("listeners"),
                                        Value::Array(
                                            item.listeners
                                                .iter()
                                                .map(|c| Value::String(c.to_string()))
                                                .collect(),
                                        ),
                                    );
                                    map
                                },
                            ));
                        }
                    }

                    for broadcast in broadcasts {
                        self.broadcast(broadcast.0, broadcast.1);
                    }
                } else {
                    println!("Unknown token {:?}", event.token());
                }

                if let Some(index) = remove_index {
                    self.clients.swap_remove(index);
                }
            }
        }
    }

    pub fn broadcast(&mut self, channel: String, data: HashMap<String, Value>) {
        println!("Broadcasting {:?} ({:?})", channel, data);
        let channel = ::channel::Channel::from(channel);
        for client in &mut self.clients {
            if client.is_listening_to(&channel) {
                client.emit(Message::Emit(channel.to_string(), data.clone()));
            }
        }
    }
}

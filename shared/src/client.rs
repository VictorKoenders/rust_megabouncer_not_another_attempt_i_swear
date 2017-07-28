use super::traits::*;
use std::net::Ipv4Addr;
use mio::net::TcpStream;
use mio::*;
use channel::Channel;
use tcp_listener::TcpReader;

pub struct Client {
    pub bundles: Vec<Box<BundleImpl>>,
    pub name: String,
}

const TOKEN: Token = Token(0);
impl Client {
    pub fn new<T: ToString>(name: T) -> Client {
        Client {
            bundles: Vec::new(),
            name: name.to_string(),
        }
    }

    pub fn register<TState: 'static>(
        &mut self,
        state: TState,
        listeners: Vec<Box<Listener<TState>>>,
    ) {
        let bundle = Bundle {
            state: state,
            listeners: listeners,
            tokens: Vec::new(),
        };
        self.bundles.push(Box::new(bundle));
    }

    pub fn run(mut self) {
        let poll = Poll::new().unwrap();
        let mut events = Events::with_capacity(1024);
        let mut next_token = 1usize;

        for bundle in &mut self.bundles {
            bundle.register_poll(&poll).unwrap();
        }
        loop {
            let stream = match TcpStream::connect(&(Ipv4Addr::new(127, 0, 0, 1), 12345).into()) {
                Ok(s) => s,
                Err(e) => {
                    println!("{:?}", e);
                    ::std::thread::sleep(::std::time::Duration::from_secs(5));
                    continue;
                }
            };

            poll.register(
                &stream,
                TOKEN,
                Ready::readable() | Ready::writable(),
                PollOpt::edge(),
            ).unwrap();
            let mut reader = TcpReader::new(stream);

            reader.write(Message::Identify(self.name.clone()));
            reader.write(Message::RegisterListeners(
                self.bundles
                    .iter()
                    .flat_map(|b| b.listener_channels())
                    .map(String::from)
                    .collect(),
            ));

            for message in self.bundles
                .iter()
                .flat_map(|b| b.connect_commands())
                .flat_map(|m| m)
            {
                reader.write(message);
            }

            while reader.connected {
                poll.poll(&mut events, None).unwrap();
                for event in &events {
                    self.handle_event(&event, &mut reader, &poll, &mut next_token);
                }
            }
            ::std::thread::sleep(::std::time::Duration::from_secs(5));
        }
    }

    fn handle_event(
        &mut self,
        event: &Event,
        reader: &mut TcpReader,
        poll: &Poll,
        next_token: &mut usize,
    ) {
        if event.token() == TOKEN {
            if event.readiness().is_writable() && reader.process_write_queue().is_err() {
                return;
            }
            if event.readiness().is_readable() {
                for message in reader.read() {
                    match message {
                        Message::Emit(str_channel, data) => {
                            let channel = Channel::from(str_channel);
                            let mut request = super::traits::Request {
                                channel: &channel,
                                data: &data,
                                responses: Vec::new(),
                                poll: poll,
                                next_token: next_token,
                                tokens: Vec::new(),
                            };
                            for bundle in &mut self.bundles {
                                bundle.handle(&mut request).unwrap();
                                if !request.tokens.is_empty() {
                                    bundle.update_tokens(Vec::new(), &mut request.tokens);
                                }
                            }
                            for message in request.responses {
                                reader.write(message);
                            }
                        }
                        x => {
                            println!("Unexpected message: {:?}", x);
                        }
                    }
                }
            }
        } else {
            let mut messages = Vec::new();
            for bundle in &mut self.bundles {
                if bundle.has_token(&event.token()) {
                    bundle.handle_event(event, &mut messages).unwrap();
                }
            }
            for message in messages {
                reader.write(message);
            }
        }
    }
}

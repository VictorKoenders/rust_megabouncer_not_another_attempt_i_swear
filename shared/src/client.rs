use super::traits::*;
use std::net::Ipv4Addr;
use mio::net::TcpStream;
use mio::*;
use channel::Channel;

pub struct Client {
    pub bundles: Vec<Box<BundleImpl>>,
    pub name: String,
}

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
        };
        self.bundles.push(Box::new(bundle));
    }

    pub fn run(mut self) {
        loop {
            let stream = match TcpStream::connect(&(Ipv4Addr::new(127, 0, 0, 1), 12345).into()) {
                Ok(s) => s,
                Err(e) => {
                    println!("{:?}", e);
                    ::std::thread::sleep(::std::time::Duration::from_secs(5));
                    continue;
                }
            };

            let poll = Poll::new().unwrap();
            const TOKEN: Token = Token(0);
            let mut events = Events::with_capacity(1024);

            poll.register(
                &stream,
                TOKEN,
                Ready::readable() | Ready::writable(),
                PollOpt::edge(),
            ).unwrap();
            let mut reader = super::tcp_listener::TcpReader::new(stream);

            reader.write(Message::Identify(self.name.clone()));
            reader.write(Message::RegisterListeners(
                self.bundles
                    .iter()
                    .flat_map(|b| b.listener_channels())
                    .map(String::from)
                    .collect(),
            ));

            while reader.connected {
                poll.poll(&mut events, None).unwrap();
                for event in &events {
                    if event.readiness().is_writable() && reader.process_write_queue().is_err() {
                        break;
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
                                        poll: &poll,
                                    };
                                    for bundle in &mut self.bundles {
                                        bundle.handle(&mut request).unwrap();
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

                }
            }
            ::std::thread::sleep(::std::time::Duration::from_secs(5));
        }
    }
}

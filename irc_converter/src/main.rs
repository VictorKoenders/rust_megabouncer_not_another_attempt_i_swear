extern crate mio;
extern crate shared;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use mio::net::TcpStream;
use shared::*;

fn main() {
    let mut client = Client::new("tcp connector");
    client.register(State::default(), vec![Box::new(Connector)]);

    client.run();
}

struct Connector;

impl Listener<State> for Connector {
    fn handle(&mut self, state: &mut State, request: &mut Request) -> Result<()> {
        if request.channel.is("data.gotten.irc_config") {
            match request.data.get("value") {
                Some(&Value::String(ref str)) => {
                    let servers: Vec<Server> = ::serde_json::from_str(str).unwrap();
                    state.merge_servers(servers, request)?;
                }
                Some(&Value::Null) => {
                    println!("No servers found");
                }
                x => {
                    println!(
                        "Could not load config from network: Expected string or null, got {:?}",
                        x
                    );
                }
            }
        } else {
            println!("Got request {:?} {:?}", request.channel, request.data);
        }
        Ok(())
    }

    fn connect_commands(&self) -> Vec<Message> {
        vec![
            Message::Emit(String::from("data.get"), {
                let mut map = HashMap::new();
                map.insert(
                    String::from("key"),
                    Value::String(String::from("irc_config")),
                );
                map
            }),
        ]
    }
    fn channels(&self) -> Vec<&str> {
        vec![
            "irc.send",
            "tcp.data",
            "tcp.status",
            "data.gotten.irc_config",
        ]
    }
}

#[derive(Default)]
struct State {
    pub connections: Vec<Connection>,
}

impl State {
    pub fn merge_servers(&mut self, servers: Vec<Server>, request: &mut Request) -> Result<()> {
        Ok(())
    }
}

#[derive(Default, Serialize, Deserialize)]
struct Server {
    pub nick: String,
    pub host: String,
    pub port: u16,
    pub use_ssl: bool,
    pub channels: Vec<Channel>,
}

#[derive(Default, Serialize, Deserialize)]
struct Channel {
    pub name: String,
    pub auto_join: bool,
}

#[derive(Default)]
struct Connection {
    pub server: Server,
    pub stream: Option<TcpStream>,
    pub channels: Vec<ConnectionChannel>,
}

#[derive(Default)]
struct ConnectionChannel {
    pub channel: String,
    pub users: Vec<UserInChannel>,
}

#[derive(Default)]
struct UserInChannel {
    pub nick: String,
    pub host: String,
    pub is_op: bool,
    pub is_voiced: bool,
}

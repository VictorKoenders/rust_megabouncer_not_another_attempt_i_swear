extern crate shared;

use std::collections::HashMap;
use shared::{Message, Value};

fn main() {
    let mut client = shared::Client::new("Logger");
    client.register(State {}, vec![Box::new(Logger {})]);

    client.run();
}

struct State;
struct Logger;

impl shared::Listener<State> for Logger {
    fn channels(&self) -> Vec<&str> {
        vec!["*"]
    }

    fn connect_commands(&self) -> Vec<Message> {
        vec![
            Message::Emit(String::from("data.get"), {
                let mut map = HashMap::new();
                map.insert(String::from("key"), Value::String(String::from("asd")));
                map
            }),
        ]
    }

    fn handle(&mut self, _state: &mut State, request: &mut shared::Request) -> shared::Result<()> {
        println!("{:?} {:?}", request.channel, request.data);
        Ok(())
    }
}

extern crate mio;
extern crate shared;

mod connector;
mod messages;
mod redis;
mod types;

pub use messages::{ReceiveMessage, SendMessage};
pub use redis::{TState, Redis};
pub use connector::Connector;
pub use types::Types;

fn main() {
    let mut client = shared::Client::new("data store");
    client.register(TState {}, vec![Box::new(Redis::default())]);

    client.run();
}

extern crate shared;

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

    fn handle(&mut self, _state: &mut State, request: &mut shared::Request) -> shared::Result<()> {
        println!("{:?} {:?}", request.channel, request.data);
        Ok(())
    }
}

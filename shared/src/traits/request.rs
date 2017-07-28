use mio::{Evented, Ready, Poll, PollOpt, Token};
use {Channel, Message, Value, Result};
use std::collections::HashMap;

pub struct Request<'a> {
    pub channel: &'a Channel,
    pub data: &'a HashMap<String, Value>,
    pub responses: Vec<Message>,
    pub poll: &'a Poll,
    pub next_token: &'a mut usize,
    pub tokens: Vec<Token>,
}

impl<'a> Request<'a> {
    pub fn send(&mut self, message: Message) {
        self.responses.push(message);
    }

    pub fn register<E>(&mut self, evented: &E) -> Result<Token>
    where
        E: Evented,
    {
        let token = Token(*self.next_token);
        *self.next_token += 1;

        self.poll.register(
            evented,
            token,
            Ready::readable() | Ready::writable(),
            PollOpt::edge(),
        )?;
        self.tokens.push(token);
        Ok(token)
    }
}

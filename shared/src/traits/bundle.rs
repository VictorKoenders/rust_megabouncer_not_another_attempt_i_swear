use super::{Listener, Message, Request, Result};
use mio::{Event, Token, Poll};

pub struct Bundle<TState: 'static> {
    pub state: TState,
    pub listeners: Vec<Box<Listener<TState>>>,
    pub tokens: Vec<Token>,
}

pub trait BundleImpl {
    fn handle(&mut self, request: &mut Request) -> Result<()>;
    fn handle_event(&mut self, event: &Event, messages: &mut Vec<Message>) -> Result<()>;
    fn listener_channels(&self) -> Vec<&str>;
    fn has_token(&self, token: &Token) -> bool;
    fn register_poll(&mut self, poll: &Poll) -> Result<()>;
    fn connect_commands(&self) -> Vec<Vec<Message>>;
    fn update_tokens(&mut self, remove_tokens: Vec<Token>, add_tokens: &mut Vec<Token>);
}

impl<TState> BundleImpl for Bundle<TState> {
    fn handle(&mut self, request: &mut Request) -> Result<()> {
        let request_channel = (*request.channel).clone();
        for listener in self.listeners.iter_mut().filter(|l| {
            l.channels().iter().any(|c| {
                let channel = ::channel::Channel::from(c);
                channel.matches(&request_channel)
            })
        })
        {
            listener.handle(&mut self.state, request)?;
        }

        Ok(())
    }

    fn listener_channels(&self) -> Vec<&str> {
        self.listeners.iter().flat_map(|l| l.channels()).collect()
    }

    fn has_token(&self, token: &Token) -> bool {
        self.tokens.iter().any(|t| t == token)
    }

    fn register_poll(&mut self, poll: &Poll) -> Result<()> {
        for listener in &mut self.listeners {
            for token in listener.register_poll(poll)? {
                self.tokens.push(token);
            }
        }
        Ok(())
    }

    fn update_tokens(&mut self, add_tokens: Vec<Token>, remove_tokens: &mut Vec<Token>) {
        self.tokens.retain(|t| !add_tokens.contains(t));
        self.tokens.append(remove_tokens);
    }

    fn connect_commands(&self) -> Vec<Vec<Message>> {
        self.listeners
            .iter()
            .map(|l| l.connect_commands())
            .collect()
    }

    fn handle_event(&mut self, event: &Event, messages: &mut Vec<Message>) -> Result<()> {
        for listener in &mut self.listeners {
            listener.handle_event(event, messages)?;
        }
        Ok(())
    }
}

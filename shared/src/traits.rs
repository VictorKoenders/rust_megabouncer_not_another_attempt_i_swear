use std::collections::HashMap;
use channel::Channel;
use mio::Poll;

pub type Result<T> = ::std::result::Result<T, ListenerError>;
#[derive(Debug)]
pub enum ListenerError {
    Unknown(String),
    FieldNotFound(String),
}

pub trait Listener<TState> {
    fn handle(&mut self, state: &mut TState, request: &mut Request) -> Result<()>;
    fn channels(&self) -> Vec<&str>;
}

pub struct Bundle<TState: 'static> {
    pub state: TState,
    pub listeners: Vec<Box<Listener<TState>>>,
}

pub trait BundleImpl {
    fn handle(&mut self, request: &mut Request) -> Result<()>;
    fn listener_channels(&self) -> Vec<&str>;
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
}

pub struct Request<'a> {
    pub channel: &'a Channel,
    pub data: &'a HashMap<String, Value>,
    pub responses: Vec<Message>,
    pub poll: &'a Poll,
}

impl<'a> Request<'a> {
    pub fn send(&mut self, message: Message) {
        self.responses.push(message);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Identify(String),
    RegisterListeners(Vec<String>),
    Emit(String, HashMap<String, Value>),
}

impl Message {
    pub fn no_name_error() -> Message {
        Message::Emit(String::from("error"), {
            let mut map = HashMap::new();
            map.insert(
                String::from("message"),
                Value::String(String::from("no name")),
            );
            map
        })
    }

    pub fn already_has_name_error() -> Message {
        Message::Emit(String::from("error"), {
            let mut map = HashMap::new();
            map.insert(
                String::from("message"),
                Value::String(String::from("already identifier")),
            );
            map
        })
    }

    pub fn is_identify(&self) -> bool {
        match *self {
            Message::Identify(_) => true,
            _ => false,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Value {
    String(String),
    I32(i32),
    F32(f32),
    Array(Vec<Value>),
}

impl Value {
    pub fn as_string(&self) -> Option<&String> {
        match *self {
            Value::String(ref str) => Some(str),
            _ => None,
        }
    }

    pub fn as_i32(&self) -> Option<&i32> {
        match *self {
            Value::I32(ref i) => Some(i),
            _ => None,
        }
    }
}

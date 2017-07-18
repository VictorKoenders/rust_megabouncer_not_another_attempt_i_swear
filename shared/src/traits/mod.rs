use mio::Event;

mod bundle;
mod message;
mod request;
mod value;

pub use self::bundle::*;
pub use self::message::*;
pub use self::request::*;
pub use self::value::*;

pub type Result<T> = ::std::result::Result<T, ListenerError>;

#[derive(Debug)]
pub enum ListenerError {
    Unknown(String),
    FieldNotFound(String),
    IOError(::std::io::Error),
}

impl From<::std::io::Error> for ListenerError {
    fn from(e: ::std::io::Error) -> ListenerError {
        ListenerError::IOError(e)
    }
}

pub trait Listener<TState> {
    fn handle(&mut self, state: &mut TState, request: &mut Request) -> Result<()>;
    fn channels(&self) -> Vec<&str>;
    fn handle_event(&mut self, _event: &Event) -> Result<()> {
        Ok(())
    }
}

use super::{TcpConnection, TcpState, ConnectionState};
use shared::{Listener, Message, Request, Result, Value};
use std::collections::HashMap;

pub struct TcpData {}

impl Listener<TcpState> for TcpData {
    fn channels(&self) -> Vec<&str> {
        vec!["tcp.send"]
    }

    fn handle(&mut self, state: &mut TcpState, request: &mut Request) -> Result<()> {
        Ok(())
    }
}

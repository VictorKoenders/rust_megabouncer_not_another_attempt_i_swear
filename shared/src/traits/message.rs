use std::collections::HashMap;
use super::Value;

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Identify(String),
    RegisterListeners(Vec<String>),
    Emit(String, HashMap<String, Value>),
}

impl Message {
    pub fn missing_key_error<T: ToString>(key: T) -> Message {
        Message::Emit(String::from("error"), {
            let mut map = HashMap::new();
            map.insert(
                String::from("message"),
                Value::String(String::from("missing key")),
            );
            map.insert(String::from("key"), Value::String(key.to_string()));
            map
        })
    }

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

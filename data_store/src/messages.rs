use shared::Value;

#[derive(Debug)]
pub enum SendMessage {
    Ping,
    Set { name: String, value: String },
    Get { name: String },
}

impl Into<Vec<u8>> for SendMessage {
    fn into(self) -> Vec<u8> {
        let mut str = match self {
            SendMessage::Ping => String::from("PING"),
            SendMessage::Set { name, value } => format!("SET {:?} {:?}", name, value),
            SendMessage::Get { name } => format!("GET {:?}", name),
        };
        str += "\r\n";
        str.into_bytes()
    }
}

#[derive(Debug)]
pub enum ReceiveMessage {
    String(String),
    Null,
    Error(String),
    Integer(i64),
    Array(Vec<ReceiveMessage>),
}

impl ReceiveMessage {
    pub fn into_value(self) -> Value {
        match self {
            ReceiveMessage::String(str) |
            ReceiveMessage::Error(str) => Value::String(str),
            ReceiveMessage::Null => Value::Null,
            ReceiveMessage::Integer(val) => Value::I32(val as i32),
            ReceiveMessage::Array(message) => Value::Array(
                message.into_iter().map(|i| i.into_value()).collect(),
            ),
        }
    }
}

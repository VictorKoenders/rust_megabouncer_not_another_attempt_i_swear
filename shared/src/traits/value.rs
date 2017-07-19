
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

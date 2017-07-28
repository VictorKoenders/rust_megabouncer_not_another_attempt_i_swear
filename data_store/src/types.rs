use std::collections::HashMap;

pub enum Types {
    Integer(i32),
    String(String),
    List(Vec<Types>),
    Set(Vec<Types>),
    Hash(HashMap<String, String>),
    Binary(Vec<u8>),
}

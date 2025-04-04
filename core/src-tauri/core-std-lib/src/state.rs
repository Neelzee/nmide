use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
    Str(String),
    List(Vec<Value>),
    Obj(HashMap<String, Value>),
}

pub type State = HashMap<String, Value>;

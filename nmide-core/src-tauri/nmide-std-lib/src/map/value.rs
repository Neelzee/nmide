use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
    String(String),
    List(Vec<Value>),
    Object(Vec<(String, Value)>),
}

impl Value {
    pub fn to_int(&self) -> Option<i32> {
        match self {
            Self::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn to_float(&self) -> Option<f32> {
        match self {
            Self::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn to_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn to_string(&self) -> Option<String> {
        match self {
            Self::String(s) => Some(s.to_string()),
            _ => None,
        }
    }

    pub fn to_list(&self) -> Option<Vec<Value>> {
        match self {
            Self::List(lst) => Some(lst.clone()),
            _ => None,
        }
    }

    pub fn to_object(&self) -> Option<Vec<(String, Value)>> {
        match self {
            Self::Object(map) => Some(map.clone()),
            _ => None,
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Int(0)
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl From<f32> for Value {
    fn from(value: f32) -> Self {
        Self::Float(value)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(value: Vec<T>) -> Self {
        Self::List(value.into_iter().map(|v| v.into()).collect::<Vec<Value>>())
    }
}

impl<S, T> From<Vec<(S, T)>> for Value
where
    S: ToString,
    T: Into<Value>,
{
    fn from(value: Vec<(S, T)>) -> Self {
        Self::Object(
            value
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.into()))
                .collect::<Vec<(String, Value)>>(),
        )
    }
}

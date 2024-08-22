use serde::{Deserialize, Serialize};

const FLOAT_COMP: f32 = 0.01f32;

#[derive(Debug, Serialize, Deserialize, Clone, PartialOrd, PartialEq)]
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

impl Eq for Value {}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Value::Int(l), Value::Int(r)) => l.cmp(r),
            (Value::Float(l), Value::Float(r)) => {
                if l - r < FLOAT_COMP {
                    std::cmp::Ordering::Equal
                } else if l - r > FLOAT_COMP {
                    std::cmp::Ordering::Greater
                } else {
                    std::cmp::Ordering::Less
                }
            }
            (Value::Bool(l), Value::Bool(r)) => l.cmp(r),
            (Value::String(l), Value::String(r)) => l.cmp(r),
            (Value::List(l), Value::List(r)) => l.cmp(r),
            (Value::Object(l), Value::Object(r)) => l.cmp(r),
            (Value::Int(l), Value::Float(_)) => Value::Float(*l as f32).cmp(other),
            (Value::Int(l), Value::Bool(r)) => std::cmp::Ordering::Less,
            (Value::Int(l), Value::String(r)) => l.cmp(&(r.len() as i32)),
            (Value::Int(l), Value::List(r)) => l.cmp(&(r.len() as i32)),
            (Value::Int(l), Value::Object(r)) => l.cmp(&(r.len() as i32)),
            (Value::Float(_), Value::Int(r)) => self.cmp(&(Value::Float(*r as f32))),
            (Value::Float(_), Value::Bool(r)) => {
                self.cmp(&(Value::Float(if *r { 1.0f32 } else { 0.0f32 })))
            }
            (Value::Float(_), Value::String(r)) => self.cmp(&(Value::Float(r.len() as f32))),
            (Value::Float(_), Value::List(r)) => self.cmp(&(Value::Float(r.len() as f32))),
            (Value::Float(_), Value::Object(r)) => self.cmp(&(Value::Float(r.len() as f32))),
            (Value::Bool(l), Value::Int(r)) => l.cmp(&(*r > 0)),
            (Value::Bool(l), Value::Float(r)) => l.cmp(&(*r > 0f32)),
            (Value::Bool(l), Value::String(r)) => l.cmp(&(r.len() > 0)),
            (Value::Bool(l), Value::List(r)) => l.cmp(&(r.len() > 0)),
            (Value::Bool(l), Value::Object(r)) => l.cmp(&(r.len() > 0)),
            (Value::String(l), Value::Int(r)) => l.cmp(&r.to_string()),
            (Value::String(l), Value::Float(r)) => l.cmp(&r.to_string()),
            (Value::String(l), Value::Bool(r)) => l.cmp(&r.to_string()),
            (Value::String(l), Value::List(r)) => l.cmp(&format!("{r:?}")),
            (Value::String(l), Value::Object(r)) => l.cmp(&format!("{r:?}")),
            (Value::List(l), Value::Int(r)) => l.cmp(&vec![(*r).into()]),
            (Value::List(l), Value::Float(r)) => l.cmp(&vec![(*r).into()]),
            (Value::List(l), Value::Bool(r)) => l.cmp(&vec![(*r).into()]),
            (Value::List(l), Value::String(r)) => l.cmp(&vec![r.clone().into()]),
            (Value::List(l), Value::Object(r)) => l.cmp(&vec![r.clone().into()]),
            (Value::Object(l), Value::Int(r)) => l.cmp(&vec![(format!("{r}"), (*r).into())].into()),
            (Value::Object(l), Value::Float(r)) => {
                l.cmp(&vec![(format!("{r}"), (*r).into())].into())
            }
            (Value::Object(l), Value::Bool(r)) => {
                l.cmp(&vec![(format!("{r}"), (*r).into())].into())
            }
            (Value::Object(l), Value::String(r)) => {
                l.cmp(&vec![(format!("{r}"), r.clone().into())].into())
            }
            (Value::Object(l), Value::List(r)) => {
                l.cmp(&vec![(format!("{r:?}"), r.clone().into())].into())
            }
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

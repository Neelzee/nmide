use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;
use crate::instruction::Instruction;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub enum Value {
    #[default]
    Null,
    Int(i32),
    Float(f32),
    Bool(bool),
    Str(String),
    List(Vec<Value>),
    Obj(HashMap<String, Value>),
}

impl Value {
    pub fn null(self) -> Option<()> {
        match self {
            Self::Null => Some(()),
            _ => None,
        }
    }

    pub fn str(self) -> Option<String> {
        match self {
            Self::Str(s) => Some(s),
            _ => None,
        }
    }

    pub fn obj(self) -> Option<HashMap<String, Value>> {
        match self {
            Self::Obj(x) => Some(x),
            _ => None,
        }
    }

    pub fn obj_add<S: ToString>(self, field: S, value: Self) -> Self {
        match &self {
            Self::Obj(mp) => {
                let mut mp = mp.clone();
                mp.insert(field.to_string(), value);
                Self::Obj(mp)
            }
            _ => self,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Int(l), Value::Int(r)) => Some(l.cmp(r)),
            (Value::Int(l), Value::Float(r)) => (*l as f32).partial_cmp(r),
            (Value::Float(l), Value::Int(r)) => l.partial_cmp(&(*r as f32)),
            (Value::Float(l), Value::Float(r)) => l.partial_cmp(r),
            (Value::Bool(l), Value::Bool(r)) => l.partial_cmp(r),
            (Value::Str(l), Value::Str(r)) => Some(l.cmp(r)),
            (Value::List(l), Value::List(r)) => l.partial_cmp(r),
            (Value::Obj(l), Value::Obj(r)) => {
                if l == r {
                    return Some(std::cmp::Ordering::Equal);
                }

                None
            }
            _ => None,
        }
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct State(HashMap<String, Value>);

impl State {
    pub fn get<S: ToString>(&self, field: S) -> Option<&Value> {
        self.0.get(&(field.to_string()))
    }

    pub fn build() -> StateInstructionBuilder {
        StateInstructionBuilder::default()
    }

    pub fn inner(self) -> HashMap<String, Value> {
        self.0
    }

    pub(crate) fn add<S: ToString>(self, field: S, value: Value) -> Self {
        let mut map = self.0;
        let field = field.to_string();

        if !field.contains(".") {
            map.insert(field, value);
            return Self(map);
        }

        let mut fields = field.split(".");

        if fields.clone().count() == 1 {
            panic!("Field cannot start or end with `.`!, {field}")
        }

        let last = fields.next().unwrap();

        let mut vec = fields.into_iter().collect::<Vec<&str>>();

        let init = {
            let mut mp = HashMap::new();
            mp.insert(vec.pop().unwrap().to_string(), value);
            Value::Obj(mp)
        };

        let obj = vec.into_iter().fold(init, |acc, c| {
            let mut mp = HashMap::new();
            mp.insert(c.to_string(), acc);
            Value::Obj(mp)
        });

        map.insert(last.to_string(), obj);

        Self(map)
    }

    pub(crate) fn rem<S: ToString>(self, field: S) -> Self {
        let mut map = self.0;
        map.remove(&field.to_string());
        Self(map)
    }

    pub(crate) fn modify<S: ToString>(self, field: S, value: Value) -> Self {
        let mut map = self.0;
        match (map.get(&field.to_string()).cloned().unwrap_or_default(), value) {
            (Value::Int(i), Value::Int(j)) => {
                map.insert(field.to_string(), Value::Int(i + j));
            },
            (Value::Float(i), Value::Int(j)) => {
                map.insert(field.to_string(), Value::Float(i + j as f32));
            },
            (Value::Float(i), Value::Float(j)) => {
                map.insert(field.to_string(), Value::Float(i + j));
            },
            (Value::Int(i), Value::Float(j)) => {
                map.insert(field.to_string(), Value::Float(i as f32 + j));
            },
            (Value::Str(i), Value::Str(j)) => {
                map.insert(field.to_string(), Value::Str(format!("{}{}", i, j)));
            },
            (Value::List(mut xs), x) => {
                xs.push(x);
                map.insert(field.to_string(), Value::List(xs));
            }
            (Value::Null, o) => {
                map.insert(field.to_string(), o);
            }
            _ => () // unimplemented modification
        }

        Self(map)
    }
}

#[derive(Default)]
// TODO: Allow for dot-notation on field access
pub struct StateInstructionBuilder(Instruction<Value>);

impl StateInstructionBuilder {
    pub fn instruction(&self) -> Instruction<Value> {
        self.0.clone()
    }

    pub(crate) fn new(instruction: Instruction<Value>) -> Self {
        Self(instruction)
    }

    pub fn add<S: ToString>(self, field: S, value: Value) -> Self {
        Self {
            0: self.0.combine(Instruction::Add(Some(field.to_string()), None, value))
        }
    }

    pub fn remove(self, field: String) -> Self {
        Self {
            0: self.0.combine(Instruction::Rem(Some(field.to_string()), None, Value::default()))
        }
    }

    pub fn set(self, field: String, value: Value) -> Self {
        self.remove(field.clone()).add(field, value)
    }

    pub fn modify(self, field: String, value: Value) -> Self {
        Self {
            0: self.0.combine(Instruction::Mod(Some(field.to_string()), None, value))
        }
    }

    // HACK: `Panic`king is done instead of having a type-level error handling, to make it
    // easier to implement
    // TODO: Make type-level error handling
    pub fn build(self, state: State) -> State {
        Self::eval(self.0, state)
    }

    fn eval(instruction: Instruction<Value>, state: State) -> State {
        match instruction {
            Instruction::Add(Some(field), _, value) => state.add(field, value),
            Instruction::Rem(Some(field), _, _) => state.rem(field),
            Instruction::Mod(Some(field), _, new_value) => state.modify(field, new_value),
            Instruction::Then(f, s) => Self::eval(*s, Self::eval(*f, state)),
            _ => state,
        }
    }
}

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Int(i32),
    Float(f32),
    Bool(bool),
    Str(String),
    List(Vec<Value>),
    Obj(HashMap<String, Value>),
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

#[derive(Debug, Default)]
pub struct State(HashMap<String, Value>);

#[derive(Default)]
pub(crate) enum StateInstruction {
    #[default]
    NoOp,
    Add {
        field: String,
        value: Value,
    },
    Set {
        field: String,
        value: Value,
    },
    Mod {
        field: String,
        value: Value,
        combine: Box<dyn Fn(Value, Value) -> Value>,
    },
    Rem {
        field: String,
    },
    Then {
        fst: Box<StateInstruction>,
        snd: Box<StateInstruction>,
    },
}

impl StateInstruction {
    pub fn combine(self, other: Self) -> Self {
        match (&self, &other) {
            (StateInstruction::NoOp, _) => other,
            (_, StateInstruction::NoOp) => self,
            _ => Self::Then {
                fst: Box::new(self),
                snd: Box::new(other),
            },
        }
    }
}

#[derive(Default)]
pub struct StateInstructionBuilder(StateInstruction);

impl StateInstructionBuilder {
    pub(crate) fn new(ins: StateInstruction) -> Self {
        Self(ins)
    }

    pub fn add(self, field: String, value: Value) -> Self {
        Self::new(self.0.combine(StateInstruction::Add { field, value }))
    }

    pub fn remove(self, field: String) -> Self {
        Self::new(self.0.combine(StateInstruction::Rem { field }))
    }

    pub fn set(self, field: String, value: Value) -> Self {
        Self::new(self.0.combine(StateInstruction::Set { field, value }))
    }

    pub fn modify<M>(self, field: String, value: Value, combine: M) -> Self
    where
        M: Fn(Value, Value) -> Value + 'static,
    {
        Self::new(self.0.combine(StateInstruction::Mod {
            field,
            value,
            combine: Box::new(combine),
        }))
    }

    // HACK: `Panic`king is done instead of having a type-level error handling, to make it
    // easier to implement
    fn _build(ins: StateInstruction, state: State) -> State {
        let mut state = state.0;
        match ins {
            StateInstruction::NoOp => State(state),
            StateInstruction::Add { field, value } => {
                if state.contains_key(&field) {
                    panic!("State already contains field: {field}, cant add it");
                }
                state.insert(field, value);
                State(state)
            }
            StateInstruction::Set { field, value } => {
                if !state.contains_key(&field) {
                    panic!("State does not contain field: {field} to set");
                }
                state.insert(field, value);
                State(state)
            }
            StateInstruction::Mod {
                field,
                value,
                combine,
            } => {
                if let Some(old_value) = state.get(&field) {
                    state.insert(field, (*combine)(value, old_value.clone()));
                    State(state)
                } else {
                    panic!("State does not contain field: {field} to modify");
                }
            }
            StateInstruction::Rem { field } => {
                if !state.contains_key(&field) {
                    panic!("State does not contain field: {field} to remove");
                }
                state.remove(&field);
                State(state)
            }
            StateInstruction::Then { fst, snd } => {
                let fst_ins = *fst;
                let snd_ins = *snd;
                Self::_build(snd_ins, Self::_build(fst_ins, State(state)))
            }
        }
    }

    // HACK: `Panic`king is done instead of having a type-level error handling, to make it
    // easier to implement
    // TODO: Make type-level error handling
    pub fn build(self, state: State) -> State {
        Self::_build(self.0, state)
    }
}

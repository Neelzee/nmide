use crate::instruction::inst::Instruction;
use crate::state::{State, Value};

#[derive(Default)]
pub struct StateBuilder(Instruction<Value>);

impl StateBuilder {
    pub fn combine(self, other: Self) -> Self {
        Self(self.0.combine(other.0))
    }

    pub fn instruction(&self) -> Instruction<Value> {
        self.0.clone()
    }

    pub(crate) fn new(instruction: Instruction<Value>) -> Self {
        Self(instruction)
    }

    pub fn add<S: ToString>(self, field: S, value: Value) -> Self {
        Self(self.0.combine(Instruction::Add(field.to_string(), value)))
    }

    pub fn remove<S: ToString>(self, field: S) -> Self {
        Self(
            self.0
                .combine(Instruction::Rem(field.to_string(), Value::default())),
        )
    }

    pub fn set<S: ToString>(self, field: S, value: Value) -> Self {
        self.remove(field.to_string().clone())
            .add(field.to_string(), value)
    }

    // HACK: `Panic`king is done instead of having a type-level error handling, to make it
    // easier to implement
    // TODO: Make type-level error handling
    pub fn build(self, state: State) -> State {
        Self::eval(self.0, state)
    }

    fn eval(instruction: Instruction<Value>, state: State) -> State {
        match instruction {
            Instruction::Add(field, value) => state.add(field, value),
            Instruction::Rem(field, _) => state.remove(field),
            Instruction::Then(f, s) => Self::eval(*s, Self::eval(*f, state)),
            _ => state,
        }
    }
}

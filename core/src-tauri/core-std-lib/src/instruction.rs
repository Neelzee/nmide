use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Default, Clone, Deserialize, Serialize, TS)]
#[serde(rename_all = "camelCase")]
pub enum Instruction<T> {
    /// No Operation, results in no change to the state
    #[default]
    NoOp,
    /// Adds the given T where the id and/or class is found.
    Add(Option<String>, Option<String>, T),
    /// Removes the given T where the id is found.
    Rem(Option<String>, Option<String>, T),
    /// Modifies the given T where the id is found.
    Mod(Option<String>, Option<String>, T),
    /// Combines two instruction into one
    Then(Box<Instruction<T>>, Box<Instruction<T>>),
}

impl<T> Instruction<T> {
    pub fn combine(self, other: Instruction<T>) -> Instruction<T> {
        match (&self, &other) {
            (Self::NoOp, _) => other,
            (_, Self::NoOp) => self,
            _ => Self::Then(Box::new(self), Box::new(other))
        }
    }
}

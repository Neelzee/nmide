use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Default, Clone, Deserialize, Serialize, TS, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum Instruction<T> {
    /// No Operation, results in no change to the state
    #[default]
    NoOp,
    /// Adds the given T where the id and/or class is found.
    Add(String, T),
    /// Removes the given T where the id is found.
    Rem(String, T),
    /// Combines two instruction into one
    Then(Box<Instruction<T>>, Box<Instruction<T>>),
}

impl<T: PartialEq> Instruction<T> {
    pub fn combine(self, other: Self) -> Self {
        match (&self, &other) {
            (Self::NoOp, _) => other,
            (_, Self::NoOp) => self,
            (Self::Add(a1, s1), Self::Rem(a2, s2)) if a1 == a2 && s1 == s2 => Self::NoOp,
            (Self::Rem(a1, s1), Self::Add(a2, s2)) if a1 == a2 && s1 == s2 => Self::NoOp,
            _ => Self::Then(Box::new(self), Box::new(other)),
        }
    }
}

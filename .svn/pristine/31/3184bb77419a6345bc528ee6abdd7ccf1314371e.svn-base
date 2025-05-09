use crate::attrs::Attr;
use crate::html::Html;
use crate::instruction::inst::Instruction;
use crate::state::Value;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

pub mod impls;

#[derive(Debug, Serialize, Deserialize, TS, Clone)]
#[ts(export)]
pub struct CoreModification {
    state: Instruction<Value>,
    ui: (Instruction<Html>, Instruction<String>, Instruction<Attr>),
}

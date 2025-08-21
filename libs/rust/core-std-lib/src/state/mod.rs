use crate::html::Html;
use hashable::HashableHashMap;
use ordered_float::NotNan;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

pub mod impls;
pub mod state_builder;
pub mod utils;

#[derive(
    Default, Debug, Clone, PartialEq, PartialOrd, Ord, Serialize, Deserialize, TS, Hash, Eq,
)]
#[serde(rename_all = "camelCase")]
#[ts(export_to = "State.ts")]
pub enum Value {
    #[default]
    Null,
    Int(i32),
    #[ts(as = "f32")]
    Float(NotNan<f32>),
    Bool(bool),
    Str(String),
    List(Vec<Value>),
    #[ts(type = "Record<string, Value | undefined>")]
    Obj(HHMap),
    Html(Html),
}

#[derive(Default, Debug, Clone, PartialEq, Hash, Eq)]
pub struct HHMap(HashableHashMap<String, Value>);

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize, TS)]
#[ts(export, type = "Record<string, Value | undefined>")]
pub struct State(HashMap<String, Value>);

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::map::value::Value;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, TS)]
#[ts(export, export_to = "../../../src/bindings/Msg.ts")]
pub enum Msg {
    Alert(String, Value),
    OpenFolderDialog(String, Value),
    PluginMsg(String, Value),
}

impl Msg {
    pub fn from_string<S>(s: S) -> Msg
    where
        S: ToString,
    {
        match s.to_string().as_str() {
            "" => Self::PluginMsg(String::new(), Value::String("".to_string())),
            s => {
                let mut parts = s.splitn(2, char::is_whitespace);
                match (parts.next(), parts.next()) {
                    (Some(a), None) => {
                        Self::PluginMsg(a.to_string(), Value::String("".to_string()))
                    }
                    (Some(l), Some(r)) => {
                        Self::PluginMsg(l.to_string(), Value::String(r.to_string()))
                    }
                    _ => unreachable!("Empty string case should be covered already"),
                }
            }
        }
    }
}

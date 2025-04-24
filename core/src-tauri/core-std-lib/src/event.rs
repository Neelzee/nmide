use crate::state::Value;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, TS)]
#[serde(rename_all = "camelCase")]
#[ts(export)]
pub struct Event {
    event: String,
    module: String,
    args: Option<Value>,
}

impl Event {
    pub fn new<S: ToString>(event_name: S, module_name: S, args: Option<Value>) -> Self {
        Self {
            event: event_name.to_string(),
            module: module_name.to_string(),
            args,
        }
    }

    pub fn event_name(&self) -> &str {
        &self.event
    }

    pub fn module_name(&self) -> &str {
        &self.module
    }

    pub fn args(&self) -> Option<&Value> {
        self.args.as_ref()
    }
}

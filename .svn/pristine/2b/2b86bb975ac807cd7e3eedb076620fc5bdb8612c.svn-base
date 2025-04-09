use serde::{Deserialize, Serialize};

use crate::state::Value;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    event_name: String,
    module_name: String,
    args: Option<Value>,
}

impl Event {
    pub fn new<S: ToString>(event_name: S, module_name: S, args: Option<Value>) -> Self {
        Self {
            event_name: event_name.to_string(),
            module_name: module_name.to_string(),
            args,
        }
    }

    pub fn event_name(&self) -> &str {
        &self.event_name
    }

    pub fn module_name(&self) -> &str {
        &self.module_name
    }

    pub fn args(&self) -> Option<&Value> {
        self.args.as_ref()
    }
}

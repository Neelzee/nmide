use serde::{Deserialize, Serialize};

use crate::state::Value;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Event {
    event_name: String,
    module_name: String,
    args: Option<Value>,
}

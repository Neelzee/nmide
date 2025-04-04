use crate::state::Value;

#[derive(Debug, PartialEq)]
pub struct Event {
    event_name: String,
    module_name: String,
    args: Option<Value>,
}

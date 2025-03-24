use crate::map::rmap::RValue;

pub struct REvent {
    event_name: String,
    module_name: String,
    args: Option<RValue>,
}

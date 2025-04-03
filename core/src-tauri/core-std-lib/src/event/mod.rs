use abi_stable::{
    std_types::{ROption, RString},
    StableAbi,
};

use crate::map::rmap::RValue;

#[repr(C)]
#[derive(StableAbi)]
pub struct REvent {
    event_name: RString,
    module_name: RString,
    args: ROption<RValue>,
}

impl REvent {
    pub fn event_name(&self) -> &str {
        &self.event_name
    }
}

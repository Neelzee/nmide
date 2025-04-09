use abi_stable::{
    StableAbi,
    std_types::{ROption, RString},
};

use crate::state::rmap::RValue;

#[repr(C)]
#[derive(StableAbi, Clone)]
pub struct REvent {
    event_name: RString,
    module_name: RString,
    args: ROption<RValue>,
}

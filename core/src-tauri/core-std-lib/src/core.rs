use abi_stable::StableAbi;

use crate::{html::rhtml::RHtml, map::rmap::RMap};

#[repr(C)]
#[derive(StableAbi)]
pub struct CoreModification;

#[repr(C)]
#[derive(StableAbi)]
pub struct Core {
    state: RMap,
    ui: RHtml,
}

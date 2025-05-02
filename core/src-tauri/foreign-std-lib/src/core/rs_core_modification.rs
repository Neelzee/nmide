use abi_stable::{std_types::RString, StableAbi};

use crate::{attr::rs_attrs::RAttr, html::rs_html::RHtml, instr::rs_instr::RInstr, state::rs_state::RValue};

#[repr(C)]
#[derive(StableAbi, Clone)]
pub struct RCoreModification {
    state: RInstr<RValue>,
    ui_html: RInstr<RHtml>,
    ui_txt: RInstr<RString>,
    ui_attr:  RInstr<RAttr>,
}
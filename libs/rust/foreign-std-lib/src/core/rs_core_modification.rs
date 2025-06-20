use std::str::FromStr;

use abi_stable::{StableAbi, std_types::RString};
use core_std_lib::core_modification::CoreModification;

use crate::{
    attr::rs_attrs::RAttr, html::rs_html::RHtml, instr::rs_instr::RInstr, state::rs_state::RValue,
};

#[repr(C)]
#[derive(StableAbi, Clone, Default)]
pub struct RCoreModification {
    state: RInstr<RValue>,
    ui_html: RInstr<RHtml>,
    ui_txt: RInstr<RString>,
    ui_attr: RInstr<RAttr>,
}

impl RCoreModification {
    pub fn add_field<S: ToString>(self, field: S, value: RValue) -> Self {
        Self {
            state: self.state.combine(RInstr::Add(
                RString::from_str(&field.to_string()).unwrap_or_default(),
                value,
            )),
            ..self
        }
    }

    pub fn add_node<S: ToString>(self, field: S, ui: RHtml) -> Self {
        Self {
            ui_html: self.ui_html.combine(RInstr::Add(
                RString::from_str(&field.to_string()).unwrap_or_default(),
                ui,
            )),
            ..self
        }
    }

    pub fn to_mod(self) -> CoreModification {
        CoreModification::from_instr(
            self.state.map(|v| v.to_value()).to_instr(),
            (
                self.ui_html.map(|h| h.to_html()).to_instr(),
                self.ui_txt.map(|t| t.as_str().to_string()).to_instr(),
                self.ui_attr.map(|a| a.to_attr()).to_instr(),
            ),
        )
    }
}

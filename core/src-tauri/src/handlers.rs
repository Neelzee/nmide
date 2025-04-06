use anyhow::Context;
use core_std_lib::{
    core::{Core, CoreModification},
    html::Html,
};
use serde::Serialize;

use crate::{
    core::NmideCore,
    statics::{COMPILE_TIME_MODULES, NMIDE_STATE, NMIDE_UI},
};

pub async fn init() -> Html {
    let modules = COMPILE_TIME_MODULES.read().await;

    let state = NmideCore.state().await;
    let ui = NmideCore.ui().await;

    let (new_state, new_ui) = modules
        .values()
        .map(|m| m.init(&NmideCore))
        .reduce(|acc, ins| acc.combine(ins))
        .unwrap_or_default()
        .build(state, ui);

    let mut st = NMIDE_STATE.write().await;
    *st = new_state;
    drop(st);
    new_ui
}

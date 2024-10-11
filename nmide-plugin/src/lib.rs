use abi_stable::{
    export_root_module,
    prefix_type::PrefixTypeTrait,
    rvec, sabi_extern_fn,
    std_types::{ROption, RString, RVec},
};
use nmide_std_lib::{
    attr::rattr::RAttr,
    html::rhtml::RHtml,
    map::rmap::RMap,
    msg::rmsg::{RMsg, RMsgKind, RMsgUnion},
    NmideStandardLibrary_Ref, NmideStdLib,
};
use std::str::FromStr;

#[export_root_module]
pub fn get_library() -> NmideStandardLibrary_Ref {
    NmideStdLib { init, view, update }.leak_into_prefix()
}

#[sabi_extern_fn]
pub fn init() -> RMap {
    RMap::new().insert("counter", 0)
}

#[sabi_extern_fn]
pub fn view(model: RMap) -> RHtml {
    let count = model
        .lookup("counter")
        .and_then(|v| ROption::RSome(v.int().unwrap_or_default()))
        .unwrap_or_default();
    RHtml::Div(
        rvec![
            RHtml::text(RString::from_str(&format!("Count: {count}")).unwrap_or_default()),
            RHtml::Button(
                RVec::new(),
                rvec![RAttr::new_click(RMsg::new(
                    RMsgKind::Msg,
                    RMsgUnion::new(RString::from_str("increment").unwrap_or_default(), 1.into())
                ))]
            )
        ],
        RVec::new(),
    )
}

#[sabi_extern_fn]
pub fn update(msg: RMsg, model: RMap) -> RMap {
    if !msg.is_msg("increment") {
        return model;
    }
    let count = model
        .lookup("counter")
        .map(|v| v.int().unwrap_or_default())
        .unwrap();
    model.insert("counter", count + 1)
}

use abi_stable::{
    export_root_module,
    prefix_type::PrefixTypeTrait,
    rvec, sabi_extern_fn,
    std_types::{ROption, RString, RVec},
};
use async_ffi::{FfiFuture, FutureExt};
use core_module_lib::rs_module::{ModuleRef, RCore_CTO, RustModule};
use core_std_lib::{event::Event, state::Value};
use foreign_std_lib::{
    attr::rs_attrs::RAttr, core::rs_core_modification::RCoreModification, event::rs_event::REvent,
    html::rs_html::RHtml, state::rs_state::RValue,
};

#[export_root_module]
pub fn get_library() -> ModuleRef {
    RustModule { init, handler }.leak_into_prefix()
}

#[sabi_extern_fn]
pub fn init(core: RCore_CTO<'static, 'static>) -> FfiFuture<()> {
    async move {
        core.send_modification(
            RCoreModification::default()
                .add_field("rs-count", RValue::new_int(1))
                .add_node(
                    "",
                    RHtml::Button(
                        RVec::new(),
                        rvec![
                            RAttr::new_click(REvent::from(Event::new(
                                "rs-count",
                                Some(Value::Int(1))
                            ))),
                            RAttr::new_id("rs-btn")
                        ],
                    )
                    .set_text("Click"),
                ),
        )
        .await;
        core.add_handler(
            RString::from("rs-count"),
            RString::from("rs-counter-plugin-example"),
        )
        .await;
    }
    .into_ffi()
}

#[sabi_extern_fn]
pub fn handler(event: REvent, core: RCore_CTO<'static, 'static>) -> FfiFuture<()> {
    async move {
        if event.event_name().to_string().as_str() == "rs-count" {
            let state = core.state().await;
            let count = state
                .lookup("rs-count")
                .and_then(|v| ROption::from(v.int()))
                .unwrap_or(0);
            core.send_modification(
                RCoreModification::default().add_field("foobar", RValue::new_int(count + 1)),
            )
            .await;
        }
    }
    .into_ffi()
}

use abi_stable::{export_root_module, prefix_type::PrefixTypeTrait, sabi_extern_fn};
use async_ffi::{FfiFuture, FutureExt};
use core_module_lib::rs_module::{ModuleRef, RCore_CTO, RustModule};
use foreign_std_lib::{
    core::rs_core_modification::RCoreModification, event::rs_event::REvent, state::rs_state::RValue,
};

#[export_root_module]
pub fn get_library() -> ModuleRef {
    RustModule { init, handler }.leak_into_prefix()
}

#[sabi_extern_fn]
pub fn init(core: RCore_CTO) -> FfiFuture<()> {
    core.send_modification(RCoreModification::default().add_field("foobar", RValue::new_int(10)))
        .into_ffi()
}

#[sabi_extern_fn]
pub fn handler(_: REvent, _: RCore_CTO) -> FfiFuture<()> {
    async move {}.into_ffi()
}

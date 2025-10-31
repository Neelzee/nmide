import gleam/utils/core

pub type JsCore

@external(javascript, "../../external/javascript_ffi.js", "from_js_core")
pub fn from_js_core_to_gleam_core(core: JsCore) -> core.Core
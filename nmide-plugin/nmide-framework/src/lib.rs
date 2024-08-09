use std::ptr::null_mut;

use nmide_rust_ffi::{html::Html, CHtml, CModel};

#[no_mangle]
pub extern "C" fn view(_: CModel) -> CHtml {
    Html::Div {
        kids: vec![Html::Text("Hello, World!".to_string())],
    }
    .to_c()
    .unwrap_or_default()
}

#[no_mangle]
pub extern "C" fn manifest() -> CModel {
    CModel { map: null_mut() }
}

use nmide_rust_ffi::{attr::Attr, html::Html, model::Model};

#[no_mangle]
pub extern "Rust" fn view(_: Model) -> Html {
    Html::Div {
        kids: vec![Html::Btn {
            kids: vec![Html::Text("FOOBAR".to_string())],
            attrs: Vec::new(),
        }],
        attrs: vec![Attr::Attr("location".to_string(), "foobar".to_string())],
    }
}

#[no_mangle]
pub extern "Rust" fn manifest() -> Model {
    let funcs: Model = vec![("nmide-functions", vec!["view"])].into();
    funcs.merge(vec![("nmide-plugin-type", "rust")].into())
}

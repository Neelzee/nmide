use nmide_rust_ffi::{
    attr::Attr,
    html::Html,
    model::{Model, Msg},
};

#[no_mangle]
pub extern "Rust" fn view(_: Model) -> Html {
    Html::Div {
        kids: vec![
            Html::Text("Hello, World!".to_string()),
            Html::Div {
                kids: Vec::new(),
                attrs: vec![Attr::OnClick(Msg::PluginMsg(
                    "Message Says: Howdy!".to_string(),
                ))],
            },
        ],
        attrs: Vec::new(),
    }
}

#[no_mangle]
pub extern "Rust" fn manifest() -> Model {
    let funcs: Model = vec![("functions", vec!["view"])].into();
    funcs.merge(vec![("rust", 1)].into())
}

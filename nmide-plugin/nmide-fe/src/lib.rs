use nmide_std_lib::{attr::Attr, html::Html, map::Map, msg::Msg};

#[no_mangle]
pub extern "Rust" fn view(_: Msg) -> Html {
    Html::Div {
        kids: vec![Html::Button {
            kids: vec![Html::Text("FOOBAR".to_string())],
            attrs: Vec::new(),
        }],
        attrs: vec![Attr::Attr("location".to_string(), "foobar".to_string())],
    }
}

#[no_mangle]
pub extern "Rust" fn manifest() -> Map {
    let funcs: Map = vec![("nmide-functions", vec!["view"])].into();
    funcs.merge(vec![("nmide-plugin-type", "rust")].into())
}

use nmide_rust_ffi::{
    attr::Attr,
    html::Html,
    map::Value,
    model::{Model, Msg},
};

#[no_mangle]
pub extern "Rust" fn view(model: Model) -> Html {
    let count = match model.lookup("counter") {
        Some(Value::Int(x)) => x.to_string(),
        _ => "No count found :(".to_string(),
    };

    Html::Div {
        kids: vec![
            Html::Text("Hello, World!".to_string()),
            Html::Text(format!("{count}")),
            Html::Btn {
                kids: vec![Html::Text("Click".to_string())],
                attrs: vec![Attr::OnClick(Msg::PluginMsg("counter".to_string()))],
            },
        ],
        attrs: vec![Attr::Id("foobar".to_string())],
    }
}

#[no_mangle]
pub extern "Rust" fn init() -> Model {
    let r: Model = vec![("counter", 0)].into();
    r.merge(vec![("nmide-plugin-framework", vec!["foobar"])].into())
}

#[no_mangle]
pub extern "Rust" fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::PluginMsg(s) => match s.as_str() {
            "counter" => match model.lookup("counter") {
                Some(Value::Int(x)) => model.insert("counter", Value::Int(x + 1)),
                Some(_) => model.insert("counter", 1.into()),
                None => model,
            },

            _ => model,
        },
    }
}

#[no_mangle]
pub extern "Rust" fn manifest() -> Model {
    let funcs: Model = vec![("nmide-functions", vec!["view", "update", "init"])].into();
    funcs.merge(vec![("nmide-plugin-type", "rust")].into())
}

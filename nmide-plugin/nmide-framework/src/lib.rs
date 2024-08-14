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
        attrs: Vec::new(),
    }
}

#[no_mangle]
pub extern "Rust" fn init() -> Model {
    vec![("counter", 0)].into()
}

#[no_mangle]
pub extern "Rust" fn update(msg: Msg, model: Model) -> Model {
    match msg {
        Msg::PluginMsg(s) => match s.as_str() {
            "counter" => match model.lookup("counter") {
                Some(Value::Int(x)) => model.insert("counter", Value::Int(x + 1)),
                _ => model,
            },

            _ => model,
        },
    }
}

#[no_mangle]
pub extern "Rust" fn manifest() -> Model {
    let funcs: Model = vec![("functions", vec!["view", "update", "init"])].into();
    funcs.merge(vec![("rust", 1)].into())
}

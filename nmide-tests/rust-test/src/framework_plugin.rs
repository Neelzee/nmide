use nmide_plugin_manager::Nmlugin;
use nmide_std_lib::{
    attr::Attr,
    html::Html,
    map::{value::Value, Map},
    msg::Msg,
};
use once_cell::sync::Lazy;
use std::path::PathBuf;
use tokio::sync::Mutex;

static PLUGIN: Lazy<Mutex<Nmlugin>> = Lazy::new(|| {
    Mutex::new(
        Nmlugin::new(
            PathBuf::from(
                "../../nmide-plugin/nmide-framework/target/release/libnmide_framework.so",
            )
            .canonicalize()
            .unwrap(),
        )
        .unwrap(),
    )
});

#[tokio::test]
async fn framework_view_test() {
    let plugin = PLUGIN.lock().await;
    let html = plugin.view(Map::new());
    assert!(html.is_ok());
    let expected_html = Html::Div {
        kids: vec![
            Html::Text("Hello, World!".to_string()),
            Html::Text("No count found :(".to_string()),
            Html::Button {
                kids: vec![Html::Text("Click".to_string())],
                attrs: vec![Attr::OnClick(Msg::PluginMsg(
                    "counter".to_string(),
                    Value::Int(0),
                ))],
            },
        ],
        attrs: vec![Attr::Id("foobar".to_string())],
    };
    assert_eq!(expected_html, html.unwrap());
}

#[tokio::test]
async fn framework_update_test() {
    let plugin = PLUGIN.lock().await;
    let model = plugin.update(
        Msg::PluginMsg("counter".to_string(), Value::Int(0)),
        Map::new(),
    );
    assert!(model.is_ok());
    let expected_model: Map = vec![("counter", Value::Int(1))].into();
    assert_eq!(expected_model, model.unwrap());
}

#[tokio::test]
async fn framework_init_test() {
    let plugin = PLUGIN.lock().await;
    let model = plugin.init();
    assert!(model.is_ok());
    let expected_model: Map = vec![("counter", Value::Int(0))].into();
    let expected_model =
        expected_model.merge(vec![("nmide-plugin-framework", vec!["foobar"])].into());
    assert_eq!(expected_model, model.unwrap());
}

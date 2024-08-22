use nmide_plugin_manager::Nmlugin;
use nmide_std_lib::{
    map::{value::Value, Map},
    msg::Msg,
};
use once_cell::sync::Lazy;
use std::path::PathBuf;
use tokio::sync::Mutex;

static PLUGIN: Lazy<Mutex<Nmlugin>> = Lazy::new(|| {
    Mutex::new(
        Nmlugin::new(
            PathBuf::from("../../nmide-plugin/nmide-manager/target/release/libnmide_manager.so")
                .canonicalize()
                .unwrap(),
        )
        .unwrap(),
    )
});

#[tokio::test]
async fn manager_manifest_test() {
    let plugin = PLUGIN.lock().await;
    let manifest = plugin.manifest();
    assert_eq!(
        manifest.lookup("nmide-plugin-type").unwrap(),
        Value::String("rust".to_string())
    );
    assert_eq!(
        manifest
            .lookup("nmide-functions")
            .and_then(|v| v.to_list())
            .unwrap()
            .sort(),
        vec![
            Value::String("view".to_string()),
            Value::String("init".to_string()),
            Value::String("update".to_string())
        ]
        .sort()
    );
}

#[tokio::test]
async fn manager_view_test() {
    let plugin = PLUGIN.lock().await;
    let html = plugin.view(Map::new());
    assert!(html.is_ok());
}

#[tokio::test]
async fn manager_update_test() {
    let plugin = PLUGIN.lock().await;
    let model = plugin.update(Msg::PluginMsg(String::new(), String::new()), Map::new());
    assert!(model.is_ok());
}

#[tokio::test]
async fn manager_init_test() {
    let plugin = PLUGIN.lock().await;
    let model = plugin.init();
    assert!(model.is_ok());
}

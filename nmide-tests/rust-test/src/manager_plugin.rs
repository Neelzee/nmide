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

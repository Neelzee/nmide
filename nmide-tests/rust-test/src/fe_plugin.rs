use nmide_plugin_manager::Nmlugin;
use nmide_std_lib::map::value::Value;
use once_cell::sync::Lazy;
use std::path::PathBuf;
use tokio::sync::Mutex;

static PLUGIN: Lazy<Mutex<Nmlugin>> = Lazy::new(|| {
    Mutex::new(
        Nmlugin::new(
            PathBuf::from("../../nmide-plugin/nmide-fe/target/release/libnmide_fe.so")
                .canonicalize()
                .unwrap(),
        )
        .unwrap(),
    )
});

#[tokio::test]
async fn fe_manifest_test() {
    let plugin = PLUGIN.lock().await;
    let manifest = plugin.manifest();
    assert_eq!(
        manifest.lookup("nmide-plugin-type").unwrap(),
        Value::String("rust".to_string())
    );
    assert_eq!(
        manifest.lookup("nmide-functions").unwrap(),
        Value::List(vec![Value::String("view".to_string())])
    );
}

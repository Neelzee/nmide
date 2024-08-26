use anyhow::{Context, Result};
use libloading::{Library, Symbol};
use nmide_plugin_manager::Nmlugin;
use nmide_std_lib::{interface::rfunctions::RManifest, map::value::Value};
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
    let lib = unsafe { Library::new(plugin.path()) };
    assert!(lib.is_ok());
    let lib = lib.unwrap();
    let func: Result<Symbol<RManifest>> = unsafe { lib.get(b"manifest") }.context("");
    assert!(func.is_ok());
    let manifest = unsafe { func.unwrap()() };
    assert_eq!(
        manifest.lookup("nmide-plugin-type").unwrap(),
        Value::String("rust".to_string())
    );
    assert_eq!(
        manifest.lookup("nmide-functions").unwrap(),
        Value::List(vec![Value::String("view".to_string())])
    );
}

use std::path::PathBuf;

use anyhow::Result;
use nmide_plugin_manager::Nmlugin;
use nmide_std_lib::{
    map::{value::Value, Map},
    msg::Msg,
    utils::consts::NMIDE_PLUGIN_LIST_KEY,
};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;

static PLUGIN_PATHS: Lazy<Mutex<Vec<Nmlugin>>> = Lazy::new(|| {
    Mutex::new(
        PathBuf::from("../../nmide-plugin/")
            .canonicalize()
            .expect("couldnt canonicalize path")
            .read_dir()
            .expect("couldnt read nmide-plugin dir")
            .into_iter()
            .filter_map(|dir| match dir {
                Ok(d) if d.path().is_dir() => Some(d.path().join("target/release").join(format!(
                    "lib{}.so",
                    d.path().file_name().unwrap().to_string_lossy().to_string().replace("-", "_")
                ))),
                Err(err) => {
                    eprintln!("Failed to get plugin path: `{err:?}`");
                    None
                }
                _ => None,
            })
            .map(|pth| {
                Nmlugin::new(&pth).expect(&format!("couldnt create plugin on path: {pth:?}"))
            })
            .collect(),
    )
});

#[tokio::test]
async fn test_init() -> Result<()> {
    // I would hate to be the guy debugging this
    let _ = PLUGIN_PATHS
        .lock()
        .await
        .iter()
        .filter_map(|p| {
            p.manifest()
                .lookup(NMIDE_PLUGIN_LIST_KEY)
                .and_then(|v| v.to_list())
                .and_then(|v| {
                    if v.contains(&Value::String("init".to_string())) {
                        println!("Init: {p:?}");
                        Some(p)
                    } else {
                        println!("No Init: {p:?}");
                        None
                    }
                })
        })
        .map(|p| p.init())
        .fold(Map::new(), |map, res| {
            assert!(res.is_ok());
            let m = res.unwrap();
            assert!(!map.overlap(&m));
            map.merge(m)
        });
    Ok(())
}

#[tokio::test]
async fn test_view() -> Result<()> {
    // I would hate to be the guy debugging this
    PLUGIN_PATHS
        .lock()
        .await
        .iter()
        .filter_map(|p| {
            p.manifest()
                .lookup(NMIDE_PLUGIN_LIST_KEY)
                .and_then(|v| v.to_list())
                .and_then(|v| {
                    if v.contains(&Value::String("view".to_string())) {
                        println!("View: {p:?}");
                        Some(p)
                    } else {
                        println!("No View: {p:?}");
                        None
                    }
                })
        })
        .map(|p| p.view(Map::new()))
        .fold(Ok(()), |_, res| {
            assert!(res.is_ok());
            Ok(())
        })
}

#[tokio::test]
async fn test_update() -> Result<()> {
    // I would hate to be the guy debugging this
    PLUGIN_PATHS
        .lock()
        .await
        .iter()
        .filter_map(|p| {
            p.manifest()
                .lookup(NMIDE_PLUGIN_LIST_KEY)
                .and_then(|v| v.to_list())
                .and_then(|v| {
                    if v.contains(&Value::String("update".to_string())) {
                        println!("Update: {p:?}");
                        Some(p)
                    } else {
                        println!("No Update: {p:?}");
                        None
                    }
                })
        })
        .map(|p| p.update(Msg::PluginMsg("".to_string(), "".to_string()), Map::new()))
        .fold(Ok(()), |_, res| {
            assert!(res.is_ok());
            Ok(())
        })
}

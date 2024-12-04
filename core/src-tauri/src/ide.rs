use crate::setup::setup;
use anyhow::Result;

pub async fn run() -> Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            setup(app).expect("Setup should always succeed");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            crate::handlers::init,
            crate::handlers::update,
            crate::handlers::view,
            crate::handlers::plugin_init,
            crate::handlers::plugin_update,
            crate::handlers::plugin_view,
            crate::handlers::get_plugins,
        ])
        .run(tauri::generate_context!())
        .expect("Application should not error");
    Ok(())
}

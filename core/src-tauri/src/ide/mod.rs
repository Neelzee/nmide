use crate::{
    core::NmideCore,
    core_modification_handler::spawn_core_modification_handler,
    ide::app::NmideApp,
    setup::setup,
    statics::{NMIDE, NMIDE_STATE, NMIDE_UI},
};
use core_std_lib::{
    core::Core, core_modification::CoreModification, event::Event, html::Html, state::Value,
};
use log::{info, warn};
use std::{collections::HashMap, path::PathBuf};
use tauri::{Emitter, Manager, RunEvent};
use tauri_plugin_cli::CliExt;

pub mod app;
pub mod setup;

#[tauri::command]
async fn init() {
    info!("[Backend] init");
    crate::handlers::init().await;
}

#[tauri::command]
async fn handler(event: Event) {
    info!("[Backend] handler, {:?}", event);
    crate::handlers::handler(event).await
}

#[tauri::command]
async fn state() -> HashMap<String, Value> {
    let st = NMIDE_STATE.read().await;
    st.clone().inner()
}

#[tauri::command]
async fn ui() -> Html {
    let ui = NMIDE_UI.read().await;
    ui.clone()
}

#[tauri::command]
async fn modification(modification: CoreModification) {
    NmideCore.send_modification(modification).await;
}

/// Runs the Tauri application
pub async fn run() {
    setup::setup_compile_time_modules()
        .await
        .expect("Compile time module setup should always succeed");

    let app = tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(tauri_plugin_log::Target::new(
                    tauri_plugin_log::TargetKind::Folder {
                        file_name: Some("out".to_string()),
                        path: PathBuf::from("../logs"),
                    },
                ))
                .build(),
        )
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            setup(setup::ide_setup(app).expect("IDE-setup should always succeed"));
            NMIDE
                .set(Box::new(NmideApp::new(app.handle().clone())))
                .unwrap_or_else(|_| panic!("AppHandle setup should always succeed"));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            init,
            state,
            ui,
            handler,
            modification
        ])
        .build(tauri::generate_context!())
        .expect("IDE Application should build successfully");

    spawn_core_modification_handler();

    app.run(move |app_handle, event| match &event {
        RunEvent::ExitRequested { .. } => app_handle
            .get_webview_window("main")
            .expect("Webview: `main` should exist")
            .destroy()
            .expect("Webview: `main` should not exist"),
        RunEvent::WindowEvent {
            event: tauri::WindowEvent::CloseRequested { api, .. },
            ..
        } => {
            app_handle
                .emit("nmide://event", Event::pre_exit())
                .expect("Emit should succeed");
            api.prevent_close();
        }
        RunEvent::Ready => match app_handle.cli().matches() {
            Ok(matches) => {
                println!("Matches: {matches:?}");
                tokio::spawn({
                    async move {
                        init().await;
                        let events: Vec<String> = matches
                            .args
                            .get("event")
                            .and_then(|a| a.value.as_array())
                            .map(|xs| {
                                xs.iter()
                                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                                    .collect()
                            })
                            .unwrap_or_default();
                        let primitive = matches
                            .args
                            .get("primitive")
                            .and_then(|a| a.value.as_bool())
                            .unwrap_or(false);
                        let args = matches
                            .args
                            .get("args")
                            .and_then(|a| a.value.as_array())
                            .cloned()
                            .unwrap_or_default()
                            .into_iter()
                            .map(|v| {
                                if primitive {
                                    let val: Result<Value, _> =
                                        serde_json::from_str(v.as_str().unwrap_or_default());
                                    return val.ok();
                                }
                                fn parse(x: serde_json::Value) -> Option<Value> {
                                    match x {
                                        serde_json::Value::Null => Some(Value::Null),
                                        serde_json::Value::Bool(b) => Some(Value::Bool(b)),
                                        serde_json::Value::Number(number) if number.is_i64() => {
                                            // WARN: This is an unsafe casting!
                                            Some(Value::Int(
                                                number.as_i64().unwrap_or_default() as i32
                                            ))
                                        }
                                        serde_json::Value::Number(number) => {
                                            // WARN: This is an unsafe casting!
                                            Some(Value::new_float(
                                                number.as_f64().unwrap_or_default() as f32,
                                            ))
                                        }
                                        serde_json::Value::String(s) if s.trim() == "!" => None,
                                        serde_json::Value::String(s) => Some(Value::Str(s)),
                                        serde_json::Value::Array(values) => Some(Value::List(
                                            values.into_iter().filter_map(parse).collect(),
                                        )),
                                        serde_json::Value::Object(map) => {
                                            Some(map.into_iter().fold(
                                                Value::new_obj(),
                                                |acc, (key, val)| {
                                                    acc.add(key, parse(val).unwrap_or_default())
                                                },
                                            ))
                                        }
                                    }
                                }
                                parse(v)
                            })
                            .collect::<Vec<_>>();
                        if args.len() > events.len() {
                            warn!("More Event Arguments given than Events!");
                        }
                        let mut events = events
                            .into_iter()
                            .zip(args)
                            .map(|(e, a)| Event::new(e, a))
                            .collect::<Vec<Event>>();
                        events.push(Event::pre_exit());
                        for e in events {
                            handler(e).await;
                        }
                    }
                });
            }
            Err(err) => {
                eprintln!("Error: {err:?}");
                app_handle.exit(0);
            }
        },
        _ => (),
    })
}

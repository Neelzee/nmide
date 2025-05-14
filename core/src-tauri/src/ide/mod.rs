use crate::{
    app::App,
    core_modification_handler::spawn_core_modification_handler,
    setup::setup,
    statics::{NMIDE, NMIDE_STATE, NMIDE_UI},
};
use core_std_lib::event::DialogFileKind::{MultiDir, MultiFile, SaveFile, SingleDir, SingleFile};
use core_std_lib::{
    core_modification::{CoreModification, UIInstr},
    event::{DialogBtn, DialogEvtKind, Event},
    html::Html,
    state::Value,
};
use log::info;
use std::{collections::HashMap, path::PathBuf};
use tauri::{AppHandle, Emitter, Manager, RunEvent};
use tauri_plugin_dialog::{DialogExt as _, MessageDialogButtons, MessageDialogKind};

mod setup;

#[tauri::command]
async fn init(mods: Vec<CoreModification>) {
    info!(place = "backend", mods:serde; "init {:?}", mods);
    let cm = mods
        .into_iter()
        .fold(CoreModification::default(), CoreModification::append);
    crate::handlers::init(cm).await;
}

#[tauri::command]
async fn handler(event: Event, mods: Vec<CoreModification>) {
    info!(place = "backend", event:serde, mods:serde; "handler, {:?} {:?}", event, mods);
    crate::handlers::handler(event, mods).await
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

pub struct NmideApp {
    handle: AppHandle,
}

#[async_trait::async_trait]
impl App for NmideApp {
    async fn rerender(&self, instr: UIInstr) {
        info!("[backend] re-render: {:?}", instr);
        self.handle
            .emit("nmide://render", instr)
            .expect("WebView should exists");
    }

    async fn event(&self, event: Event) {
        let app = self.handle.clone();
        match event {
            Event::DialogEvent {
                event,
                kind,
                message,
                btn,
                title,
            } => {
                let mut dia = app.dialog().message(message);

                dia = if let Some(t) = title {
                    dia.title(t)
                } else {
                    dia
                };

                dia = match kind {
                    Some(DialogEvtKind::Info) => dia.kind(MessageDialogKind::Info),
                    Some(DialogEvtKind::Warning) => dia.kind(MessageDialogKind::Warning),
                    Some(DialogEvtKind::Error) => dia.kind(MessageDialogKind::Error),
                    _ => dia,
                };

                dia = match btn {
                    Some(DialogBtn::Ok) => dia.buttons(MessageDialogButtons::Ok),
                    Some(DialogBtn::OkCancel) => dia.buttons(MessageDialogButtons::OkCancel),
                    Some(DialogBtn::OkCancelCustom(x, y)) => {
                        dia.buttons(MessageDialogButtons::OkCancelCustom(x, y))
                    }
                    Some(DialogBtn::OkCustom(x)) => dia.buttons(MessageDialogButtons::OkCustom(x)),
                    Some(DialogBtn::YesNo) => dia.buttons(MessageDialogButtons::YesNo),
                    None => todo!(),
                };

                dia.show(move |result| {
                    app.emit(
                        "nmide://event",
                        Event::core_response(event, Some(Value::Bool(result))),
                    )
                    .expect("Emitting event should succeed");
                });
            }
            Event::DialogFile {
                event,
                title,
                file_kind,
                filter_ext,
                create_dirs,
            } => {
                let mut dia = app.dialog().file();
                dia = if let Some(t) = title {
                    dia.set_title(t)
                } else {
                    dia
                };

                dia = dia.set_can_create_directories(create_dirs);

                dia = dia.add_filter(
                    format!("{event}-filter"),
                    &filter_ext.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
                );

                match file_kind {
                    SingleFile => dia.pick_file(move |fp| {
                        let file = fp
                            .map(|x| x.as_path().map(|y| y.to_path_buf()))
                            .and_then(|p| p.map(|x| x.to_str().map(|s| s.to_string())))
                            .and_then(|s| s.map(|x| Value::Str(x)));
                        app.emit("nmide://event", Event::core_response(event, file))
                            .expect("Emitting event should succeed");
                    }),
                    SingleDir => dia.pick_folder(move |fp| {
                        let file = fp
                            .map(|x| x.as_path().map(|y| y.to_path_buf()))
                            .and_then(|p| p.map(|x| x.to_str().map(|s| s.to_string())))
                            .and_then(|s| s.map(|x| Value::Str(x)));
                        app.emit("nmide://event", Event::core_response(event, file))
                            .expect("Emitting event should succeed");
                    }),
                    MultiFile => dia.pick_files(move |fp| {
                        let files = fp
                            .and_then(|xs| {
                                xs.into_iter()
                                    .map(|x| x.as_path().map(|y| y.to_path_buf()))
                                    .map(|x| x.and_then(|p| p.to_str().map(|s| s.to_string())))
                                    .map(|s| s.map(|x| Value::Str(x)))
                                    .collect()
                            })
                            .map(|xs| Value::List(xs));
                        app.emit("nmide://event", Event::core_response(event, files))
                            .expect("Emitting event should succeed");
                    }),
                    MultiDir => dia.pick_folders(move |fp| {
                        let files = fp
                            .and_then(|xs| {
                                xs.into_iter()
                                    .map(|x| x.as_path().map(|y| y.to_path_buf()))
                                    .map(|x| x.and_then(|p| p.to_str().map(|s| s.to_string())))
                                    .map(|s| s.map(|x| Value::Str(x)))
                                    .collect()
                            })
                            .map(|xs| Value::List(xs));
                        app.emit("nmide://event", Event::core_response(event, files))
                            .expect("Emitting event should succeed");
                    }),
                    SaveFile => todo!(),
                };
            }
            e => {
                app.emit("nmide://event", e).expect("WebView should exists");
            }
        }
    }

    async fn exit(&self) {
        self.handle.exit(0);
    }
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
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            NMIDE
                .set(Box::new(NmideApp {
                    handle: app.handle().clone(),
                }))
                .unwrap_or_else(|_| panic!("AppHandle setup should always succeed"));
            setup(setup::ide_setup(app).expect("IDE-setup should always succeed"));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![init, state, ui, handler])
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
        _ => (),
    })
}

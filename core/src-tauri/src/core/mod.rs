use crate::statics::{MODULE_EVENT_REGISTER, NMIDE, NMIDE_SENDER, NMIDE_STATE, NMIDE_UI};
use async_trait::async_trait;
use core_std_lib::event::DialogFileKind::{MultiDir, MultiFile, SaveFile, SingleDir, SingleFile};
use core_std_lib::event::{DialogBtn, DialogEvtKind};
use core_std_lib::state::Value;
use core_std_lib::{
    core::Core, core_modification::CoreModification, event::Event, html::Html, state::State,
};
use log::info;
use std::collections::HashMap;
use tauri::Emitter;
use tauri_plugin_dialog::{DialogExt, MessageDialogButtons, MessageDialogKind};
use tokio::sync::{mpsc::Sender, RwLock};

#[cfg(feature = "runtime_modules")]
pub mod runtime_core;

#[derive(Default)]
pub struct ModuleEventRegister {
    event: RwLock<HashMap<String, Vec<String>>>,
}

impl ModuleEventRegister {
    pub async fn get_module_names(&self, event: &Event) -> Vec<String> {
        let mut modules = Vec::new();

        modules.append(
            &mut self
                .event
                .read()
                .await
                .get(event.event_name())
                .cloned()
                .unwrap_or(Vec::new()),
        );

        modules
    }

    pub async fn register_module(&mut self, event: String, handler: String) {
        info!(
            "[backend][handler-register] register module: {}, to event {:?}",
            handler, event
        );
        let mut modules = self.event.write().await;
        let mut vec = modules.get(&event).cloned().unwrap_or(Vec::new());
        vec.push(handler.clone());
        modules.insert(event, vec);
    }
}

pub struct NmideCore;

#[async_trait]
impl Core for NmideCore {
    async fn state(&self) -> State {
        info!("[backend] state");
        let st = NMIDE_STATE.read().await;
        st.clone()
    }

    async fn ui(&self) -> Html {
        info!("[backend] ui");
        let ui = NMIDE_UI.read().await;
        ui.clone()
    }

    async fn throw_event(&self, event: Event) {
        let app = NMIDE
            .get()
            .expect("AppHandle should be initialized")
            .read()
            .await;
        match event {
            e @ Event::Event { .. } => {
                app.emit("nmide://event", e)
                    .expect("AppHandle emit should always succeed");
            }
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
            Event::PostInit | Event::PreExit | Event::CoreResponse { .. } => (),
        }
    }

    async fn add_handler(&self, event_name: String, handler_name: String) {
        let mut reg = MODULE_EVENT_REGISTER.write().await;
        reg.register_module(event_name, handler_name).await;
    }

    async fn send_modification(&self, modification: CoreModification) {
        NMIDE_SENDER
            .get()
            .expect("Sender should be initialized")
            .send(modification)
            .await
            .expect("Channel should be opened");
    }
}

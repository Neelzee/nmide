use crate::app::App;
use core_std_lib::{
    core_modification::UIInstr,
    event::{
        DialogBtn, DialogEvtKind,
        DialogFileKind::{MultiDir, MultiFile, SaveFile, SingleDir, SingleFile},
        Event,
    },
    state::Value,
};
use log::info;
use tauri::{AppHandle, Emitter};
use tauri_plugin_dialog::{DialogExt as _, MessageDialogButtons, MessageDialogKind};

pub struct NmideApp {
    handle: AppHandle,
}

impl NmideApp {
    pub fn new(handle: AppHandle) -> Self {
        Self { handle }
    }
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
                            .and_then(|s| s.map(Value::Str));
                        app.emit("nmide://event", Event::core_response(event, file))
                            .expect("Emitting event should succeed");
                    }),
                    SingleDir => dia.pick_folder(move |fp| {
                        let file = fp
                            .map(|x| x.as_path().map(|y| y.to_path_buf()))
                            .and_then(|p| p.map(|x| x.to_str().map(|s| s.to_string())))
                            .and_then(|s| s.map(Value::Str));
                        app.emit("nmide://event", Event::core_response(event, file))
                            .expect("Emitting event should succeed");
                    }),
                    MultiFile => dia.pick_files(move |fp| {
                        let files = fp
                            .and_then(|xs| {
                                xs.into_iter()
                                    .map(|x| x.as_path().map(|y| y.to_path_buf()))
                                    .map(|x| x.and_then(|p| p.to_str().map(|s| s.to_string())))
                                    .map(|s| s.map(Value::Str))
                                    .collect()
                            })
                            .map(|xs| -> Value { Value::List(xs) });
                        app.emit("nmide://event", Event::core_response(event, files))
                            .expect("Emitting event should succeed");
                    }),
                    MultiDir => dia.pick_folders(move |fp| {
                        let files = fp
                            .and_then(|xs| {
                                xs.into_iter()
                                    .map(|x| x.as_path().map(|y| y.to_path_buf()))
                                    .map(|x| x.and_then(|p| p.to_str().map(|s| s.to_string())))
                                    .map(|s| s.map(Value::Str))
                                    .collect()
                            })
                            .map(|xs| -> Value { Value::List(xs) });
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

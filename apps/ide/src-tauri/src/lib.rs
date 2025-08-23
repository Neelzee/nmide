use crate::{
    context::platform::TauriPlatform,
};
use log::debug;
use tokio::sync::mpsc::channel;
use anyhow::{Context, Result};
use core_std_lib::{
    core::Core, core_modification::CoreModification, html::Html, state::Value,
    state::State,
};
use std::fs;
use tauri::{Emitter, Manager, RunEvent};
use std::{collections::HashMap, path::PathBuf};
use crate::{
    context::{compile_time::NmideCore, runtime::RuntimeCore},
};
use abi_stable::sabi_trait::TD_Opaque;
use core_module_lib::rs_module::RCore_CTO;
use core_std_lib::{event::Event};
use foreign_std_lib::event::rs_event::REvent;
use futures::{self, FutureExt};
use log::{error, info, warn};
use crate::{context::event_register::ModuleEventRegister, context::platform::Platform};
use core_module_lib::{rs_module::RsModule, Module};
use once_cell::sync::{Lazy, OnceCell};
use tokio::sync::{mpsc::Sender, RwLock};

pub mod context {
    pub mod platform {

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

        #[derive(Debug)]
        pub struct TauriPlatform {
            handle: AppHandle,
        }

        #[async_trait::async_trait]
        impl Platform for TauriPlatform {
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

        impl TauriPlatform {
            pub fn new(handle: AppHandle) -> Self {
                Self { handle }
            }
        }

        #[async_trait::async_trait]
        pub trait Platform: Send + Sync + std::fmt::Debug {
            /// Emits a re-render notification
            async fn rerender(&self, instr: UIInstr);
            /// Emits an Event
            async fn event(&self, event: Event);
            /// Exits the application
            async fn exit(&self);
        }
    }
    pub mod event_register {
        use core_std_lib::event::Event;
        use log::info;
        use std::collections::HashMap;
        use tokio::sync::RwLock;

        /// Holds a mapping between modules and the Events they have registered for.
        #[derive(Default)]
        pub struct ModuleEventRegister {
            /// Event name → List of Module names
            event: RwLock<HashMap<String, Vec<String>>>,
        }

        impl ModuleEventRegister {
            /// Retrievs all the modules that has registered for this Event. This
            /// includes modules who has registered for the `*` event.
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

                modules.append(
                    &mut self
                        .event
                        .read()
                        .await
                        .get("*")
                        .cloned()
                        .unwrap_or(Vec::new()),
                );

                modules
            }

            pub async fn register_module(&mut self, event: String, handler: String) {
                info!(
                    place = "backend";
                    "register module: {}, to event {:?}",
                    handler, event
                );
                let mut modules = self.event.write().await;
                let mut vec = modules.get(&event).cloned().unwrap_or(Vec::new());
                vec.push(handler.clone());
                modules.insert(event, vec);
            }
        }

    }
    pub mod runtime {
        use crate::{
            context::compile_time::NmideCore,
            NMIDE_SENDER, NMIDE_STATE, NMIDE_UI,
        };
        use abi_stable::std_types::RString;
        use async_ffi::{FfiFuture, FutureExt};
        use core_module_lib::rs_module::RCore;
        use core_std_lib::core::Core;
        use foreign_std_lib::{
            core::rs_core_modification::RCoreModification, event::rs_event::REvent, html::rs_html::RHtml,
            state::rs_state::RState,
        };
        use log::{error, info};

        pub struct RuntimeCore;

        impl RCore for RuntimeCore {
            extern "C" fn state(&self) -> FfiFuture<RState> {
                info!("[rt-core] state");
                async move {
                    let state = NMIDE_STATE.read().await.clone();
                    RState::from(state)
                }
                .into_ffi()
            }

            extern "C" fn ui(&self) -> FfiFuture<RHtml> {
                info!("[rt-core] ui");
                async move {
                    let ui = NMIDE_UI.read().await.clone();
                    RHtml::from(ui)
                }
                .into_ffi()
            }

            extern "C" fn throw_event(&self, event: REvent) -> FfiFuture<()> {
                let evt = event.to_event();
                info!("[rt-core] event: {evt:?}");
                NmideCore.throw_event(evt).into_ffi()
            }

            extern "C" fn add_handler(&self, event_name: RString, handler_name: RString) -> FfiFuture<()> {
                let evt_name = event_name.as_str().to_string();
                let hand_name = handler_name.as_str().to_string();
                info!("[rt-core] {hand_name} -> {evt_name}");
                NmideCore.add_handler(evt_name, hand_name).into_ffi()
            }

            extern "C" fn send_modification(&self, modification: RCoreModification) -> FfiFuture<()> {
                async move {
                    let result =
                        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| modification.to_mod()));

                    let mods = match result {
                        Ok(mods) => {
                            info!("[rt-core] modification: {mods:?}");
                            mods
                        }
                        Err(err) => {
                            error!("[rt-core] Panic occurred in modification.to_mod(), error: {err:#?}");
                            return;
                        }
                    };

                    match NMIDE_SENDER.get() {
                        Some(sender) => match sender.send(mods).await {
                            Ok(_) => {
                                info!("[rt-core] Modification sent successfully");
                            }
                            Err(err) => {
                                error!("[rt-core] Error sending modification: {err:?}");
                            }
                        },
                        None => {
                            error!("[rt-core] NMIDE_SENDER not initialized");
                        }
                    }
                }
                .into_ffi()
            }
        }

    }
    pub mod compile_time {

        use crate::{
            APP_DATA_DIR, MODULE_EVENT_REGISTER, NMIDE, NMIDE_SENDER, NMIDE_STATE, NMIDE_UI,
        };
        use async_trait::async_trait;
        use core_std_lib::{
            core::Core, core_modification::CoreModification, event::Event, html::Html, state::State,
        };
        use log::info;
        use std::path::PathBuf;

        /// Compile-time core
        ///
        /// Implements the `Core` trait, for use by compile-time modules.
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
                NMIDE
                    .get()
                    .expect("AppHandle should be initialized")
                    .event(event)
                    .await;
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

            async fn appdir(&self) -> PathBuf {
                APP_DATA_DIR
                    .get()
                    .expect("Should be initialized")
                    .read()
                    .await
                    .as_path()
                    .to_path_buf()
            }
        }
    }
}

/// HashMap, mapping module names to their corresponding runtime-module, is
/// protected by a `RwLock`, so is thread safe.
pub static RUNTIME_MODULES: tokio::sync::OnceCell<RwLock<HashMap<String, RsModule>>> =
    tokio::sync::OnceCell::const_new();

/// Path to the runtime module directory, currently not used, but could be used
/// in the future to allow for post-startup additions of modules, by "watching"
/// this folder.
pub static RUNTIME_MODULE_DIR: tokio::sync::OnceCell<PathBuf> = tokio::sync::OnceCell::const_new();

/// Path to the directory "owned" by this application, is used across instances,
/// so could be used by modules for long-term application specific values, but
/// this functionality is currently not exposed to modules.
pub static APP_DATA_DIR: OnceCell<RwLock<PathBuf>> = OnceCell::new();

/// Thread safe representation of the webview. Should be representative, but
/// changes made to the webview outside of the application, i.e. by modules
/// directly adding elements with `createElement`, are not dected. Used by
/// modules for "reading" the Html-tree.
pub static NMIDE_UI: Lazy<RwLock<Html>> = Lazy::new(|| RwLock::new(Html::Main()));

/// Thread safe state of the application, can only be changed by using the
/// application itself, so will always be representative of what is actually
/// happening. A copy is accessible by modules.
pub static NMIDE_STATE: Lazy<RwLock<State>> = Lazy::new(|| RwLock::new(State::default()));

/// Thread safe hashmap, mapping module names to a Module implementation. Not
/// accessible by other modules.
pub static COMPILE_TIME_MODULES: Lazy<RwLock<HashMap<String, Box<dyn Module>>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

/// Thread safe ModuleEventRegister, a manager struct for storing the mapping
/// between modules, and events they are triggered by. Not accessible by other
/// modules
pub static MODULE_EVENT_REGISTER: Lazy<RwLock<ModuleEventRegister>> =
    Lazy::new(|| RwLock::new(ModuleEventRegister::default()));

/// Thread safe `Sender`, a struct for sending `CoreModification`s. Is only
/// directly accessed by the Core, which clones it before passing it to
/// modules.
pub static NMIDE_SENDER: tokio::sync::OnceCell<Sender<CoreModification>> =
    tokio::sync::OnceCell::const_new();

/// Thread safe AppHandle. Used because some processes need to `emit` events,
/// which is a one-way method for the backend to communicate with the frontend.
pub static NMIDE: tokio::sync::OnceCell<Box<dyn Platform>> = tokio::sync::OnceCell::const_new();

pub async fn _init() {
    let rt_modules = RUNTIME_MODULES
        .get()
        .expect("Should be initialized at this point")
        .read()
        .await;
    let rt_module_futures = rt_modules
        .values()
        .map(|m| m.init(RCore_CTO::from_const(&RuntimeCore, TD_Opaque)))
        .collect::<Vec<_>>();
    let modules = COMPILE_TIME_MODULES.read().await;
    let module_futures = modules.values().map(|m| m.init(Box::new(NmideCore)));
    futures::future::join_all(module_futures).await;

    for f in rt_module_futures {
        match f.catch_unwind().await {
            Ok(_) => (),
            Err(err) => error!("[backend] panic: {err:?}"),
        }
    }

    NmideCore.throw_event(Event::PostInit).await;
}

pub async fn _handler(event: Event) {
    let evt = event.clone();
    tokio::spawn({
        async move {
            let evt = event.clone();
            let mut revt = None;
            let mods = COMPILE_TIME_MODULES.read().await;
            let rt_mods = RUNTIME_MODULES
                .get()
                .expect("Should be initialized")
                .read()
                .await;
            let mut modules = Vec::new();
            let mut rt_modules = Vec::new();
            let triggered_modules = MODULE_EVENT_REGISTER
                .read()
                .await
                .get_module_names(&evt)
                .await;
            info!(place = "backend", event:serde, triggered_modules:serde; "Handler, {:?} {:?}", event, triggered_modules);
            for mod_name in triggered_modules {
                if let Some(m) = mods.get(&mod_name) {
                    modules.push(m.handler(evt.clone(), Box::new(NmideCore)));
                }

                if let Some(m) = rt_mods.get(&mod_name) {
                    if revt.is_none() {
                        revt = Some(REvent::from(evt.clone()));
                    }
                    rt_modules.push(m.handler(
                        revt.clone().unwrap(),
                        RCore_CTO::from_const(&RuntimeCore, TD_Opaque),
                    ));
                }
            }
            futures::future::join_all(modules).await;
            for f in rt_modules {
                match f.catch_unwind().await {
                    Ok(_) => (),
                    Err(err) => error!("[backend] panic: {err:?}"),
                }
            }
        }
    });

    if matches!(evt, Event::PreExit) {
        info!(place = "backend"; "Exiting");
        NMIDE
            .get()
            .expect("AppHandle should be initialized")
            .exit()
            .await;
    }
}

fn ide_setup(app: &mut tauri::App) -> Result<(PathBuf, PathBuf)> {
    let app_handle = app.app_handle();
    Ok((
        app_handle.path().app_data_dir()?,
        app_handle.path().app_data_dir()?.join("modules"),
    ))
}

#[allow(unused_imports, unused_variables)]
pub mod module_reg {
    use core_module_lib::Module;
    use core_module_lib::ModuleBuilder;
    use core_std_lib::core::Core;
    use std::collections::HashMap;
    include!(concat!(env!("OUT_DIR"), "/module_reg.rs"));
}

async fn setup_compile_time_modules() -> Result<()> {
    let mut modules: HashMap<String, Box<dyn Module>> = HashMap::new();

    module_reg::register_modules(&mut modules);

    let str_modules = modules
        .values()
        .map(|m| (*m).name().to_string())
        .collect::<Vec<String>>();
    info!("compile-time modules: {:?}", str_modules,);

    let mut m = COMPILE_TIME_MODULES.write().await;
    *m = modules;

    Ok(())
}


#[tauri::command]
async fn init() {
    info!("[Backend] init");
    _init().await
}

#[tauri::command]
async fn handler(event: Event) {
    info!("[Backend] handler, {:?}", event);
    _handler(event).await
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


/// Ensures the static variables are initialized before used.
///
/// See [static](crate::static)
///
/// # Panics
///
/// Panics if $APPDATA, $APPCACHE or $APPDATA/plugins does not exist.
pub fn _setup(paths: (PathBuf, PathBuf)) {
    let (app_data, nmide_module) = paths;

    APP_DATA_DIR
        .set(RwLock::new(app_data))
        .expect("Initialization of APP_DATA_DIR should always succeed");

    RUNTIME_MODULE_DIR
        .set(nmide_module)
        .expect("Initialization of NMIDE_PLUGIN_DIR should always succeed");

    let nmide_module_dir = RUNTIME_MODULE_DIR.get().unwrap();
    if !nmide_module_dir.exists() {
        fs::create_dir_all(nmide_module_dir)
            .unwrap_or_else(|err| {
                panic!("Creation of the module directory: `{nmide_module_dir:?}` should succeed, failed with error: {err:?}")
            });
    }

    RUNTIME_MODULES.set(RwLock::new(
        nmide_module_dir
            .read_dir()
            .unwrap_or_else(|err| {
                panic!("Reading the module directory: `{nmide_module_dir:?}` should succeed, failed with error: {err:?}")
            })
            .filter_map(|dir| match dir {
                Ok(d)
                if d.path().is_file()
                && d.path().extension().is_some_and(|e| {
                // TODO: This will not work, need a cfg for os
                    e.to_string_lossy() == "so" || e.to_string_lossy() == "dll"
                }) =>
                {
                    info!("RT-Module: {:?}", d.path());
                    Some(d.path())
                }
                Err(err) => {
                    warn!("Failed to get module path: `{err:?}`");
                    None
                }
                _ => None,
            })
            .filter_map(|pth| {
                match RsModule::new(pth.as_path()) {
                    Ok(rm) => Some(rm),
                    Err(err) => {
                    warn!("Could not create module on path: {pth:?}, due too {err:?}");
                        None
                    },
                }
            })
            .map(|m| {
                info!("[backend] RT-Module: {}", m.name());
                (m.name(), m)
            })
            .collect()
    )).expect("Reading from the plugin directory should not fail");
}


/// Runs the Tauri application
pub async fn run() -> Result<usize> {
    setup_compile_time_modules().await;
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
            _setup(ide_setup(app).expect("IDE-setup should always succeed"));
            NMIDE
                .set(Box::new(TauriPlatform::new(app.handle().clone())))
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
        .context("IDE Application should build successfully")?;

    spawn_core_modification_handler();

    let exitcode = app.run_return(move |app_handle, event| match &event {
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
    });

    Ok(exitcode as usize)
}

/// Spawns the thread handling Core Modifications.
///
/// Initializes `NMIDE_SENDER`, so should only be invoked once.
///
/// # Panics
///
/// - If `NMIDE_SENDER` already has been set.
/// - If `NMIDE` has not been set.
pub fn spawn_core_modification_handler() {
    tokio::spawn({
        let (sender, mut recv) = channel::<CoreModification>(100);
        NMIDE_SENDER.set(sender).expect("NMIDE_SENDER not set yet");
        async move {
            while let Some(pre_modification) = recv.recv().await {
                let modification = pre_modification.clone().optimize();
                let state = NMIDE_STATE.read().await.clone();
                let ui = NMIDE_UI.read().await.clone();

                let (new_state, ui_builder) = modification.clone().build_state(state);
                let mut st = NMIDE_STATE.write().await;
                *st = new_state;
                let state = st.clone();
                let inst = ui_builder.instruction();
                let mut current_ui = NMIDE_UI.write().await;
                *current_ui = ui_builder.build(ui);
                let ui = current_ui.clone();
                let platform = NMIDE.get().expect("Platform should be initialized");
                debug!(
                    place = "backend",
                    state:serde,
                    ui:serde,
                    pre_modification:serde,
                    pre_len = pre_modification.len(),
                    post_len = modification.len(),
                    modification:serde;
                    "received modification {:?} {:?} {:?}",
                    state,
                    ui,
                    modification
                );
                platform.rerender(inst).await;
            }
        }
    });
}

use crate::{
    app::App,
    core::NmideCore,
    core_modification_handler::spawn_core_modification_handler,
    ide::setup,
    statics::{COMPILE_TIME_MODULES, MODULE_EVENT_REGISTER, NMIDE},
};
use anyhow::Result;
use core_std_lib::{core::Core, core_modification::UIInstr, event::Event, state::Value};
use log::{info, warn};
use tokio::{
    io::{self, AsyncReadExt},
    sync::RwLock,
};

static EXIT_CLI: RwLock<bool> = RwLock::const_new(false);

pub async fn run() -> Result<()> {
    env_logger::init();
    let _ = NMIDE.set(Box::new(CliApp));
    setup::setup_compile_time_modules().await?;
    spawn_core_modification_handler();
    let mut buff = String::new();
    let mut stdin = io::stdin();
    println!("Any input will be handled as an `Event::CoreResponse`");
    loop {
        if *EXIT_CLI.read().await {
            break;
        }
        buff.clear();
        print!("> ");
        let _ = stdin.read_to_string(&mut buff).await;
        NmideCore
            .throw_event(Event::core_response(
                "stdin",
                Some(Value::Str(buff.clone())),
            ))
            .await;
    }

    Ok(())
}

pub struct CliApp;

#[async_trait::async_trait]
impl App for CliApp {
    async fn rerender(&self, _: UIInstr) {}

    async fn event(&self, event: Event) {
        info!("Recieved event: {event:?}");
        match event {
            Event::DialogEvent { .. } => {}
            Event::DialogFile { .. } => {}
            e => {
                tokio::spawn({
                    async move {
                        let register = MODULE_EVENT_REGISTER.read().await;
                        let modules = register.get_module_names(&e).await;
                        info!("Event: {e:?}, Modules: {modules:?}");
                        let module_handlers = COMPILE_TIME_MODULES.read().await;
                        let mut futures = Vec::new();
                        for m in modules {
                            if let Some(ctm) = module_handlers.get(&m) {
                                futures.push(ctm.handler(e.clone(), Box::new(NmideCore)));
                            } else {
                                warn!("No module with name `{m}` found, maybe misspelt?");
                            }
                        }
                        futures::future::join_all(futures).await;
                    }
                });
            }
        }
    }

    async fn exit(&self) {
        *EXIT_CLI.write().await = true;
    }
}

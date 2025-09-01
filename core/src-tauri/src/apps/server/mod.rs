use crate::apps::App as NmideApp;
use crate::core::setup::setup as core_setup;
use crate::core::statics::COMPILE_TIME_MODULES;
use actix_files::{self as fs};
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use core_std_lib::core::Core;
use core_std_lib::core_modification::CoreModification;
use core_std_lib::event::Event;
use core_std_lib::html::Html;
use core_std_lib::state::State;
use env_logger::Env;
use serde::{Deserialize, Serialize};

pub struct Server;

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonCore {
    state: State,
    ui: Html,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonCoreResponse {
    modification: CoreModification,
    events: Vec<Event>,
    handler: Vec<(String, String)>,
}

#[post("/modules/{module}/init")]
async fn module_init(path: web::Path<String>, json_core: web::Json<JsonCore>) -> HttpResponse {
    let module_name = path.into_inner();
    let modules = COMPILE_TIME_MODULES.read().await;
    if let Some(module) = modules.get(&module_name) {
        let body = serde_json::to_string(&JsonCoreResponse {
            modification: todo!(),
            events: todo!(),
            handler: todo!(),
        })
        .unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[async_trait]
impl NmideApp for Server {
    async fn setup() -> Result<()> {
        let env = Env::default().filter_or("NMIDE_LOG_LEVEL", "info");
        env_logger::init_from_env(env);
        core_setup(("./static/".into(), "./static/".into()));
        Ok(())
    }

    async fn run() -> Result<usize> {
        let srv = HttpServer::new(|| {
            App::new()
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i"))
                .service(module_init)
                .service(fs::Files::new("/", "./static").show_files_listing())
        })
        .bind(("0.0.0.0", 8080))
        .expect("Port should be available")
        .run();

        match srv.await {
            Ok(()) => Ok(0),
            Err(err) => Err(anyhow!("Error exiting server: {err:?}")),
        }
    }
}

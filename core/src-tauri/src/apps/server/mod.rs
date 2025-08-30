use crate::apps::App as NmideApp;
use actix_files::{self as fs};
use actix_web::middleware::Logger;
use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_static_files::ResourceFiles;
use anyhow::{anyhow, Result};

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

pub struct Server;

#[async_trait::async_trait]
impl NmideApp for Server {
    async fn setup() -> Result<()> {
        env_logger::init();
        Ok(())
    }

    async fn run() -> Result<usize> {
        let srv = HttpServer::new(|| {
            let generated = generate();
            App::new()
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i"))
                .service(echo)
                .service(fs::Files::new("/static", "./static").show_files_listing())
                .service(ResourceFiles::new("/", generated))
                .route("/hey", web::get().to(manual_hello))
        })
        .bind(("127.0.0.1", 8080))
        .expect("Port should be available")
        .run();

        match srv.await {
            Ok(()) => Ok(0),
            Err(err) => Err(anyhow!("Error exiting server: {err:?}")),
        }
    }
}

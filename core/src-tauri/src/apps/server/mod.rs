use crate::apps::App as NmideApp;
use anyhow::{Result, anyhow};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files::{NamedFile, self as fs};
use actix_web::middleware::Logger;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open("./static/index.html").ok()
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
            App::new()
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i"))
                .service(echo)
                .service(fs::Files::new("/static", "./static").show_files_listing())
                .service(index)
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

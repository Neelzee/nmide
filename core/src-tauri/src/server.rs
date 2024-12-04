use actix_web::{get, App, HttpResponse, HttpServer};

use crate::handlers;

#[get("/init")]
pub async fn init() -> HttpResponse {
    HttpResponse::Ok().json(handlers::init().await)
}

pub async fn run() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(init))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;
    Ok(())
}

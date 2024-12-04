use std::{path::PathBuf, str::FromStr};

use crate::{handlers, setup::setup};
use actix_web::{get, middleware, web, App, HttpResponse, HttpServer};
use anyhow::Result;
use nmide_std_lib::{map::tmap::TMap, msg::tmsg::TMsg};

#[get("/init")]
pub async fn init() -> HttpResponse {
    HttpResponse::Ok().json(handlers::init().await)
}

#[get("/view")]
pub async fn view(tmodel: web::Json<TMap>) -> HttpResponse {
    HttpResponse::Ok().json(handlers::view(tmodel.0).await)
}

#[get("/update")]
pub async fn update(tmsg: web::Json<TMsg>, tmodel: web::Json<TMap>) -> HttpResponse {
    HttpResponse::Ok().json(handlers::update(tmsg.0, tmodel.0).await)
}

pub async fn run() -> Result<()> {
    setup(server_setup().expect("Server Setup should succeed")).expect("Setup should succeed");
    HttpServer::new(|| App::new().service(init).wrap(middleware::Logger::default()))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await?;
    Ok(())
}

fn server_setup() -> Result<(PathBuf, PathBuf, PathBuf)> {
    Ok((
        PathBuf::from_str("./app_data")?,
        PathBuf::from_str("./app_cache")?,
        PathBuf::from_str("./app_data/plugins")?,
    ))
}

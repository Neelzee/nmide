use std::{path::PathBuf, str::FromStr};

use crate::{handlers, setup::setup};
use actix_cors::Cors;
use actix_web::{middleware, options, post, web, App, HttpResponse, HttpServer};
use anyhow::Result;
use core_std_lib::{map::tmap::TMap, msg::tmsg::TMsg};

#[post("/init")]
pub async fn init() -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json(handlers::init().await)
}

#[post("/view")]
pub async fn view(tmodel: web::Json<TMap>) -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json(handlers::view(tmodel.0).await)
}

#[post("/update")]
pub async fn update(tmsg: web::Json<TMsg>, tmodel: web::Json<TMap>) -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json(handlers::update(tmsg.0, tmodel.0).await)
}

pub async fn run() -> Result<()> {
    setup(server_setup().expect("Server Setup should succeed")).expect("Setup should succeed");
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::default().allow_any_origin())
            .service(init)
            .service(view)
            .service(update)
    })
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

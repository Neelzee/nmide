//! The Server-module contains the necessary code to turn the IDE into a web-IDE, in
//! theory. In practice there are a lot of things left to be solved, if one wants to use this as an
//! web-IDE, especially if the aim is to provide this web-IDE for multiple users.
//!
//! This module, under the [server](crate::server) feature, exposes three POST endpoints, similar to
//! the [ide](crate::ide) module: [init](crate::handlers::init), [update](crate::handlers::update)
//! and [view](crate::handlers::view). The _input_ and _output_ of these endpoints are of the same
//! type as in the [ide](crate::ide) module.

use crate::{handlers, setup::setup};
use actix_cors::Cors;
use actix_web::{middleware, post, web, App, HttpResponse, HttpServer};
use anyhow::Result;
use core_std_lib::{map::tmap::TMap, msg::tmsg::TMsg};
use std::{path::PathBuf, str::FromStr};

/// see [init](crate::handlers::init)
#[post("/init")]
pub async fn init() -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json(handlers::init().await)
}

/// see [view](crate::handlers::view)
#[post("/view")]
pub async fn view(tmodel: web::Json<TMap>) -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json(handlers::view(tmodel.0).await)
}

/// see [update](crate::handlers::update)
#[post("/update")]
pub async fn update(tmsg: web::Json<TMsg>, tmodel: web::Json<TMap>) -> HttpResponse {
    HttpResponse::Ok()
        .insert_header(("Access-Control-Allow-Origin", "*"))
        .json(handlers::update(tmsg.0, tmodel.0).await)
}

/// Runs the server
///
/// # Panics
/// If the [`server_setup`](crate::server::server_setup) fails.
///
/// # Errors
/// - If [bind](actix_web::server::HttpServer::bind) fails
/// - if [run](actix_web::server::HttpServer::run) fails
pub async fn run() -> Result<()> {
    setup(server_setup().expect("Server Setup should succeed"));
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

/// Gets appdata path, and plugin directory
#[allow(
    missing_docs,
    reason = "Cannot error on PathBuf construction, due to constant, valid, input"
)]
fn server_setup() -> Result<(PathBuf, PathBuf)> {
    Ok((
        PathBuf::from_str("./app_data")?,
        PathBuf::from_str("./app_data/plugins")?,
    ))
}

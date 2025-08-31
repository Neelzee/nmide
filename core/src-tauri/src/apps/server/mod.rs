use crate::apps::App as NmideApp;
use actix_files::{self as fs};
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use anyhow::{anyhow, Result};
use env_logger::Env;

pub struct Server;

#[async_trait::async_trait]
impl NmideApp for Server {
    async fn setup() -> Result<()> {
        let env = Env::default().filter_or("NMIDE_LOG_LEVEL", "info");
        env_logger::init_from_env(env);
        Ok(())
    }

    async fn run() -> Result<usize> {
        let srv = HttpServer::new(|| {
            App::new()
                .wrap(Logger::default())
                .wrap(Logger::new("%a %{User-Agent}i"))
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

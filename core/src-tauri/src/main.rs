// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[allow(unused_imports)]
use anyhow::Context;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    if cfg!(feature = "ide") {
        #[cfg(feature = "ide")]
        core_lib::ide::run()
            .await
            .context("An error occurred when running the IDE")?;
    } else if cfg!(feature = "server") {
        #[cfg(feature = "server")]
        core_lib::server::run()
            .await
            .context("An error occurred when running the Server")?;
    } else {
        panic!("Invalid state, specify either ide or server as features");
    }
    Ok(())
}

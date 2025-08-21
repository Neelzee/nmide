// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod ide;
mod core;
mod setup;
mod statics;
mod handlers;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    ide::run().await;
    Ok(())
}

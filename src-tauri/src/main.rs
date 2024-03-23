// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::workspace::Workspace;
use eyre::{Context, Result};
#[warn(unused_imports)]
use log::{info, warn};
use once_cell::sync::Lazy;
use std::path::Path;
use tokio::sync::Mutex;

mod cmds;
mod errors;
mod osops;
#[cfg(test)]
mod test;
mod types;
mod utils;
mod workspace;

pub static WORKSPACE: Lazy<Mutex<Workspace>> = Lazy::new(|| {
    info!("Initializing new workspace");
    // TODO: should use empty
    #[cfg(windows)]
    return Mutex::new(
        Workspace::init(Path::new("C:\\Users\\nilsi\\Documents\\nmide"))
            .expect("Failed opening root"),
    );

    #[cfg(not(windows))]
    return Mutex::new(
        Workspace::init(Path::new("/home/nmf/Documents/nmide")).expect("Failed opening root"),
    );
});

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}! You've been greeted from Rust!")
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let g = WORKSPACE.try_lock().wrap_err("Failed")?;

    println!("{g:?}");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    Ok(())
}

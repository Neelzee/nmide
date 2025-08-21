// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use tokio::io::{self, AsyncReadExt};

#[tokio::main]
async fn main() -> Result<()> {
    let args = std::env::args();
    for a in args {
        if a.contains("--cli") {
            loop {
                println!("Cli");
                core_lib::cli::run().await?;
                let mut buff = String::new();
                let mut stdin = io::stdin();
                println!("[1] Restart CLI");
                println!("[2] Continue to App");
                println!("[3] Exit");
                stdin.read_to_string(&mut buff).await.unwrap();
                match buff.trim().as_ref() {
                    "1" => {
                        println!("Restarting CLI");
                        continue;
                    },
                    "2" => {
                        println!("Starting App");
                        break;
                    },
                    "3" => {
                        println!("Exiting");
                        return Ok(());
                    },
                    x => {
                        println!("Unknown command: `{x}`, exiting");
                        return Ok(());
                    },
                }
            }
        }
    }
    core_lib::ide::run().await;
    Ok(())
}

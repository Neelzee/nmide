// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process;

use anyhow::Result;
use core_lib::apps::App;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = core_lib::apps::tui::cli::cmd().get_matches();

    match matches.subcommand() {
        Some(("install", _)) => {
            #[cfg(not(feature = "module-installer"))]
            {
                eprintln!("Need feature `module-installer` to be enabled");
                process::exit(1);
            }
            #[cfg(feature = "module-installer")]
            {
                #[cfg(debug_assertions)]
                env_logger::init();
                /*
                                match core_lib::installer::install_modules().await {
                                    Ok(_) => {
                                        println!("Finished installing modules");
                                        if cfg!(debug_assertions) {
                                            println!("Cargo.toml may have changed, exiting");
                                            process::exit(0);
                                        }
                                    }
                                    Err(err) => {
                                        eprintln!("Something went wrong during installation: {err:?}");
                                        process::exit(1);
                                    }
                                }
                */
            }
        }
        Some(("clean", _)) => {
            #[cfg(not(feature = "module-installer"))]
            {
                eprintln!("Need feature `module-installer` to be enabled");
                process::exit(1);
            }
            #[cfg(feature = "module-installer")]
            {
                #[cfg(debug_assertions)]
                env_logger::init();
                /*match core_lib::installer::clean_modules().await {
                    Ok(_) => {
                        println!("Finished cleaning modules");
                        if cfg!(debug_assertions) {
                            println!("Cargo.toml may have changed, exiting");
                            process::exit(0);
                        }
                    }
                    Err(err) => {
                        eprintln!("Something went wrong during cleaning: {err:?}");
                        process::exit(1);
                    }
                }*/
            }
        }
        Some(("server", _)) => {
            #[cfg(feature = "server")]
            {
                print!("Setting up server...");
                core_lib::apps::server::Server::setup().await.unwrap();
                println!("✓");
                println!("Running server");
                let _ = core_lib::apps::server::Server::run().await.unwrap();
                return Ok(());
            }
            #[cfg(not(feature = "server"))]
            {
                eprintln!("Need `server` feature to be enabled.");
                process::exit(1);
            }
        }
        _ => (),
    }
    core_lib::apps::desktop::DesktopApp::setup().await?;
    let _exitcode = core_lib::apps::desktop::DesktopApp::run().await?;
    Ok(())
}

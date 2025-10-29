// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(feature = "ide"))]
use anyhow::anyhow;
use anyhow::Result;
use clap::{Arg, Command};
use std::process;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cmd().get_matches();

    match matches.subcommand() {
        Some(("install", _)) => {
            #[cfg(not(feature = "module-installer"))]
            {
                eprintln!("Need feature `module-installer` to be enabled");
                process::exit(1);
            }
            #[cfg(feature = "module-installer")]
            {
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
                match core_lib::installer::clean_modules().await {
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
                }
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

    #[cfg(feature = "ide")]
    {
        core_lib::apps::desktop::DesktopApp::setup().await?;
        let _exitcode = core_lib::apps::desktop::DesktopApp::run().await?;
    }

    #[cfg(not(feature = "ide"))]
    {
        eprintln!("Need ide feature enabled");
        Err(anyhow!("Need ide feature enabled"))
    }?;

    Ok(())
}

pub fn cmd() -> Command {
    Command::new("cmi")
        .about("Nmide Module Installer")
        .subcommand(Command::new("install").about("Installs modules").alias("i"))
        .subcommand(
            Command::new("clean")
                .about("Removes installed modules")
                .alias("c"),
        )
        .subcommand(Command::new("server").about("Runs the web IDE"))
}

pub fn add_args(cmd: Command) -> Command {
    cmd.arg(
        Arg::new("cargo")
            .long("cargo")
            .help("Cargo.toml path")
            .num_args(1)
            .required(true),
    )
    .arg(
        Arg::new("modules")
            .long("modules")
            .help("Path to modules folder")
            .num_args(1)
            .required(true),
    )
    .arg(
        Arg::new("conf")
            .long("conf")
            .help("Path to modules folder")
            .num_args(1)
            .required(true),
    )
    .arg(
        Arg::new("out")
            .long("out")
            .help("Path to target folder")
            .num_args(1)
            .required(true),
    )
    .arg(
        Arg::new("dist")
            .long("dist")
            .help("Path to build folder")
            .num_args(1)
            .required(true),
    )
    .arg(
        Arg::new("index")
            .long("index")
            .num_args(1)
            .help("Index.html path")
            .required(true),
    )
    .arg(
        Arg::new("appdir-modules")
            .long("appdir-modules")
            .num_args(1)
            .help("Appdir path"),
    )
    .arg(
        Arg::new("dry-run")
            .long("dry-run")
            .num_args(0)
            .help("Prints the result of running this command, without running it"),
    )
}

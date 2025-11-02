// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[cfg(not(feature = "ide"))]
use anyhow::anyhow;
use anyhow::Result;
use clap::{Arg, Command};
use std::process;
#[cfg(feature = "ide")]
use core_lib::apps::App;
#[cfg(feature = "server")]
use core_lib::apps::App;

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cmd().get_matches();

    match matches.subcommand() {
        Some(("install", arg)) => {
            #[cfg(not(feature = "module-installer"))]
            {
                eprintln!("Need feature `module-installer` to be enabled");
                process::exit(1);
            }
            #[cfg(feature = "module-installer")]
            {

                let module_folder: String = arg.get_one("appdir-modules").cloned().unwrap();

                core_lib::installer::installer::install(
                    arg.get_one("conf").cloned().unwrap_or(env!("MODULE_CONFIG")),
                    arg.get_one("cargo").cloned().unwrap_or(env!("CARGO_PATH")),
                    arg.get_one("modules").cloned().unwrap_or(env!("MODULES")),
                    arg.get_one("out").cloned().unwrap_or(env!("OUT")),
                    arg.get_one("index").cloned().unwrap_or(env!("INDEX_PATH")),
                    false,
                    &module_folder,
                    arg.get_flag("dry-run"),
                    arg.get_one("dist").cloned().unwrap_or(env!("DIST_DIR")),
                );
                println!("Finished installing modules");
                if cfg!(debug_assertions) {
                    println!("Modules might have changed, exiting");
                    process::exit(0);
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
        .subcommand(
            add_args(
            Command::new("install").about("Installs modules").alias("i")
            )
        )
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
    )
    .arg(
        Arg::new("modules")
            .long("modules")
            .help("Path to modules folder")
            .num_args(1)
    )
    .arg(
        Arg::new("conf")
            .long("conf")
            .help("Path to modules folder")
            .num_args(1)
    )
    .arg(
        Arg::new("out")
            .long("out")
            .help("Path to target folder")
            .num_args(1)
    )
    .arg(
        Arg::new("dist")
            .long("dist")
            .help("Path to build folder")
            .num_args(1)
    )
    .arg(
        Arg::new("index")
            .long("index")
            .num_args(1)
            .help("Index.html path")
    )
    .arg(
        Arg::new("appdir-modules")
            .long("appdir-modules")
            .num_args(1)
            .help("Appdir path")
            .required(true),
    )
    .arg(
        Arg::new("dry-run")
            .long("dry-run")
            .num_args(0)
            .help("Prints the result of running this command, without running it"),
    )
}

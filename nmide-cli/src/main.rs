use std::path::PathBuf;

use anyhow::Result;
use clap::{Arg, Command};

mod code_gen;
fn cli() -> Command {
    Command::new("ncli")
        .about("Nmide CLI")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("new")
                .arg(
                    Arg::new("path")
                        .short('p')
                        .long("path")
                        .value_name("PATH")
                        .help("path to plugin"),
                )
                .arg(
                    Arg::new("name")
                        .short('n')
                        .long("name")
                        .value_name("NAME")
                        .help("plugin name"),
                )
                .arg_required_else_help(false),
        )
}

#[tokio::main]
async fn main() -> Result<()> {
    let matches = cli().get_matches();
    let path = match matches.subcommand() {
        Some(("new", arg)) if arg.contains_id("path") => arg.clone().remove_one("path").unwrap(),
        _ => ".",
    };
    let name = match matches.subcommand() {
        Some(("name", arg)) if arg.contains_id("name") => arg.clone().remove_one("name").unwrap(),
        _ => ".",
    };

    let final_path = PathBuf::new().join(path).join(name).canonicalize()?;

    tokio::fs::create_dir_all(&final_path).await?;

    Ok(())
}

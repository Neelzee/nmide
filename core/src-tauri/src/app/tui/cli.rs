use clap::{Arg, Command};

pub fn cmd() -> Command {
    Command::new("cmi")
        .about("Nmide Module Installer")
        .subcommand(Command::new("install").about("Installs modules").alias("i"))
        .subcommand(
            Command::new("clean")
                .about("Removes installed modules")
                .alias("c"),
        )
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

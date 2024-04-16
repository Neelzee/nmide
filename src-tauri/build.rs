use dotenv::dotenv;
use eyre::Result;
use std::{env, fs};

#[cfg(windows)]
include!("src\\nmide\\utils\\consts.rs");
#[cfg(not(windows))]
include!("src/nmide/utils/consts.rs");

fn main() -> Result<()> {
    setup()?;
    tauri_build::build();

    Ok(())
}

fn setup() -> Result<()> {
    dotenv().ok();

    let path = env::var(NMIDE_CONF_PATH)?;

    if !fs::metadata(&path).is_ok() {
        fs::create_dir_all(&path)?;
    }

    let conf_path = format!("{path}/{NMIDE_CONF_FILE}");
    if !fs::metadata(&conf_path).is_ok() {
        fs::File::create(conf_path)?;
    }

    Ok(())
}

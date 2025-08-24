use crate::model::Module;
use anyhow::{Context, Result, anyhow};
use std::{path::PathBuf, str::FromStr};
use tokio::process::Command;

async fn cargo_check(path: &PathBuf) -> Result<()> {
    Command::new("cargo")
        .current_dir(path)
        .arg("check")
        .status()
        .await
        .context(format!(
            "Failed executing cmd: cargo check, in pwd: {path:?}",
        ))
        .and_then(|s| {
            s.code().ok_or(anyhow!(
                "Failed getting ExitStatus: cargo check, in pwd: {path:?}",
            ))
        })
        .and_then(|s| {
            if s == 0 {
                Ok(())
            } else {
                Err(anyhow!(
                    "Failed command: cargo check, in pwd: {path:?}, with exitcode: {s}"
                ))
            }
        })
}

pub async fn install(module: Module) -> Result<String> {
    let path = PathBuf::from_str(&module.path)?;
    cargo_check(&path).await?;
    /* What TODO: Next:
     * 1. Add this module to the Cargo.toml file
     * 2. Add this module to imports in `module_reg.rs`
     * 3. Add this module to the method in `module_reg.rs`
     */
    Ok(module.name)
}

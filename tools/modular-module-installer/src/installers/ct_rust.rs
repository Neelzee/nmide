use crate::model::Module;
use anyhow::{Context, Result, anyhow};
use std::path::PathBuf;
use tokio::process::Command;

async fn cargo_build(path: &PathBuf, build_output: &PathBuf) -> Result<i32> {
    Command::new("cargo")
        .current_dir(path)
        .arg("build")
        .arg("--release")
        .arg("--target-dir")
        .arg(build_output)
        .status()
        .await
        .context(format!(
            "Failed executing cmd: cargo build --release --target-dir {:?}, in pwd: {path:?}",
            build_output
        ))
        .and_then(|s| {
            s.code()
                .ok_or(anyhow!("Failed getting ExitStatus: cargo build --release --target-dir {:?}, in pwd: {path:?}",
            build_output
            ))
        })
}

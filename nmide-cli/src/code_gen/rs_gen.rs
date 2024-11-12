use std::path::PathBuf;

use super::CodeGen;
use anyhow::{anyhow, Context, Result};
use askama::Template;
use tokio::process::Command;

#[derive(Template)]
#[template(path = "rs_lib.html")]
struct RsTemplate {
    cargo: CargoTemplate,
}

#[derive(Template)]
#[template(path = "rs_cargo.html")]
struct CargoTemplate {
    plugin_name: String,
}

impl CodeGen for RsTemplate {
    fn new(plugin_name: String) -> Self {
        Self {
            cargo: CargoTemplate { plugin_name },
        }
    }

    async fn code_gen(&self) -> Result<()> {
        let lib_content = self.render().context("Failed to generate template")?;
        let cargo_content = self
            .cargo
            .render()
            .context("Failed to generate Cargo template")?;

        Command::new("cargo")
            .arg("new")
            .arg("--lib")
            .arg(&self.cargo.plugin_name)
            .status()
            .await
            .context(format!(
                "Failed executing command: cargo new --lib {}",
                &self.cargo.plugin_name,
            ))
            .and_then(|status| {
                if status.success() {
                    Ok(())
                } else {
                    Err(anyhow!("Failed with ExitStatus: {status}"))
                }
            })?;

        tokio::fs::write(
            PathBuf::from(&self.cargo.plugin_name).join("src/lib.rs"),
            lib_content.as_bytes(),
        )
        .await
        .context("Failed writing template to src/lib.rs")?;

        tokio::fs::write(
            PathBuf::from(&self.cargo.plugin_name).join("Cargo.toml"),
            cargo_content.as_bytes(),
        )
        .await
        .context("Failed writing template to src/lib.rs")?;

        Ok(())
    }

    async fn compile_cmd(&self) -> Result<()> {
        Command::new("cargo")
            .arg("build")
            .arg("--release")
            .status()
            .await
            .context("Failed executing command: cargo build --release")
            .and_then(|status| {
                if status.success() {
                    Ok(())
                } else {
                    Err(anyhow!("Failed with ExitStatus: {status}"))
                }
            })
    }

    async fn move_to_plugins(&self, path: std::path::PathBuf) -> Result<()> {
        todo!()
    }
}

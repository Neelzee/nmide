use crate::app::App;
use anyhow::{Context, Result};

pub mod run;
pub mod setup;

pub struct DesktopApp;

#[async_trait::async_trait]
impl App for DesktopApp {
    async fn setup() -> Result<()> {
        setup::setup_compile_time_modules()
            .await
            .context("Compile time module setup should always succeed")?;
        Ok(())
    }
    async fn run() -> Result<usize> {
        run::run().await
    }
}

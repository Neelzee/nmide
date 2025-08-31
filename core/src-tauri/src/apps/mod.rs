use anyhow::Result;

#[cfg(feature = "ide")]
pub mod desktop;
#[cfg(feature = "server")]
pub mod server;
pub mod tui;

#[async_trait::async_trait]
pub trait App {
    async fn setup() -> Result<()> {
        Ok(())
    }
    async fn run() -> Result<usize>;
}

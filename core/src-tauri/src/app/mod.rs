use anyhow::Result;

pub mod desktop;
pub mod tui;

#[async_trait::async_trait]
pub trait App {
    async fn setup() -> Result<()> {
        Ok(())
    }
    async fn run() -> Result<usize>;
}

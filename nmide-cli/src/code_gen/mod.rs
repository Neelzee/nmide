use anyhow::Result;
use std::path::PathBuf;

mod js_gen;
mod rs_gen;

pub trait CodeGen {
    fn new(plugin_name: String) -> Self;

    async fn code_gen(&self) -> Result<()>;

    async fn compile_cmd(&self) -> Result<()>;

    async fn move_to_plugins(&self, path: PathBuf) -> Result<()>;
}

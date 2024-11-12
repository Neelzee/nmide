use super::CodeGen;
use anyhow::{Context, Result};
use askama::Template;

#[derive(Template)]
#[template(path = "js.html")]
struct JsTemplate {
    plugin_name: String,
}

impl CodeGen for JsTemplate {
    fn new(plugin_name: String) -> Self {
        Self { plugin_name }
    }

    async fn code_gen(&self) -> Result<()> {
        let file_content = self.render().context("Failed to generate template")?;

        Ok(())
    }

    async fn compile_cmd(&self) -> Result<()> {
        Ok(())
    }

    async fn move_to_plugins(&self, path: std::path::PathBuf) -> Result<()> {
        todo!()
    }
}

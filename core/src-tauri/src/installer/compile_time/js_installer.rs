use crate::installer::{
    types::{ModuleConfig, ModuleInstaller},
    utils::{run_cmd, DIST_DIR},
};
use anyhow::Result;
use log::info;
use std::{fs, path::PathBuf, process::Command};

#[derive(Clone, Copy)]
pub struct JsInstaller;

#[async_trait::async_trait]
impl ModuleInstaller for JsInstaller {
    async fn install(&self, modules: &[(String, ModuleConfig)]) -> Result<()> {
        info!("JSM compile-time installer");
        let mut imports = Vec::new();
        for (name, m) in modules {
            if !m.enabled {
                continue;
            }
            if !matches!(m.kind.as_str(), "ts" | "mjs" | "js") {
                continue;
            }

            info!("Installing JS module: {name}");

            let mut path: PathBuf = m.path.clone().into();

            let pm = m.package_manager.clone().unwrap_or("bun".to_string());

            let mut install_cmd = Command::new(pm.clone());
            install_cmd.current_dir(&path);
            install_cmd.arg("i");
            run_cmd(install_cmd);
            let mut build_cmd = Command::new(pm);
            build_cmd.current_dir(&path);
            build_cmd.arg("run");
            build_cmd.arg("build");
            run_cmd(build_cmd);
            path = path.join("build/index.js");
            let import = format!("import '{}'", path.as_os_str().to_str().unwrap());
            imports.push(import);
        }

        let s: String = imports.join("\n");

        fs::write(format!("{DIST_DIR}/modules.js"), s)
            .inspect_err(|err| panic!("Failed to write to file: {DIST_DIR}, error: {err:?}"))
            .unwrap();
        Ok(())
    }
}

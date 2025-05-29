use anyhow::{Context, Result};
use compile_time::{js_installer::JsInstaller, rs_installer::RsInstaller};
use log::{debug, info};
use types::{CoreModuleConfig, ModuleInstaller};

pub mod compile_time;
pub mod types;
pub(crate) mod utils;

pub async fn clean_modules() -> Result<()> {
    info!("Cleaning modules");
    if cfg!(debug_assertions) {
        let installers: Vec<Box<dyn ModuleInstaller>> =
            vec![Box::new(JsInstaller), Box::new(RsInstaller)];
        for installer in installers.iter() {
            installer.install(&Vec::new()).await?;
        }
    }

    // TODO: Add runtime module cleaning

    Ok(())
}

pub async fn install_modules() -> Result<()> {
    let installers: Vec<Box<dyn ModuleInstaller>> = Vec::new();

    let config_str = include_str!("../../../../Modules.toml");
    let config: CoreModuleConfig = toml::from_str(config_str)
        .context(format!("Failed to parse Modules.toml: {config_str}"))?;

    info!(
        "Found {} modules",
        config.modules.len() + config.rt_modules.len()
    );

    debug!(
        "JS Modules: {:?}",
        config
            .modules
            .iter()
            .filter(|(_, m)| matches!(m.kind.as_str(), "ts" | "js" | "mjs"))
            .map(|(s, _)| s)
            .chain(
                config
                    .rt_modules
                    .iter()
                    .filter(|(_, m)| matches!(m.kind.as_str(), "ts" | "js" | "mjs"))
                    .map(|(s, _)| s)
            )
            .collect::<Vec<_>>()
    );

    debug!(
        "RS Modules: {:?}",
        config
            .modules
            .iter()
            .filter(|(_, m)| matches!(m.kind.as_str(), "rs"))
            .map(|(s, _)| s)
            .chain(
                config
                    .rt_modules
                    .iter()
                    .filter(|(_, m)| matches!(m.kind.as_str(), "rs"))
                    .map(|(s, _)| s)
            )
            .collect::<Vec<_>>()
    );

    if cfg!(debug_assertions) {
        let installers: Vec<Box<dyn ModuleInstaller>> =
            vec![Box::new(JsInstaller), Box::new(RsInstaller)];
        // HACK: This is a really stupid way to get the path stuff to _work_.
        // Should be fixed before release.
        let mods = config
            .modules
            .into_iter()
            .map(|(s, mut m)| {
                if m.kind.as_str() == "rs" {
                    m.path = format!("../../{}", m.path.clone());
                    (s, m)
                } else {
                    m.path = format!("../../../{}", m.path.clone());
                    (s, m)
                }
            })
            .collect::<Vec<_>>();
        for installer in installers.iter() {
            installer.install(&mods).await?;
        }
    }

    let mods = config.rt_modules.into_iter().collect::<Vec<_>>();

    for installer in installers {
        installer.install(&mods).await?;
    }

    Ok(())
}

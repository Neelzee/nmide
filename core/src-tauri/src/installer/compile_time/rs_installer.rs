use crate::installer::{
    types::{ModuleConfig, ModuleInstaller},
    utils::MODULE_SEPARATOR,
};
use anyhow::{Context, Result};
use log::info;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
};
use toml::Value;

#[derive(Debug, Deserialize, Serialize)]
struct Module {
    pub(crate) path: String,
    #[serde(default = "Vec::new")]
    pub(crate) features: Vec<String>,
}

impl Module {
    fn from_conf(module: &ModuleConfig) -> Self {
        Self {
            path: module.path.clone(),
            features: module.features.clone(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct RsInstaller;

#[async_trait::async_trait]
impl ModuleInstaller for RsInstaller {
    async fn install(&self, modules: &[(String, ModuleConfig)]) -> Result<()> {
        info!("RSM compile-time installer");
        let mut module_imports = Vec::new();
        let mut module_reg = Vec::new();
        let cargo_path = Path::new(env!("CARGO_PATH")).canonicalize().unwrap();
        let mut tbl = toml::Table::new();
        let mut mods = Vec::new();
        for (module, m) in modules {
            if !m.enabled || !matches!(m.kind.as_str(), "rs") {
                continue;
            }

            let mut t = toml::Table::new();
            info!("Installing module: {}", module);
            t.insert(
                module.clone(),
                toml::from_str(&toml::to_string(&Module::from_conf(m))?)?,
            );
            mods.push(t);
            module_imports.push(format!("use {};", module));
            module_reg.push(format!(
                "modules.insert(\"{}\".to_string(), Box::new({}::ModuleBuilder.build()));",
                module, module
            ));
        }
        let is_empty = mods.is_empty();
        tbl.insert(
            "dependencies".to_string(),
            Value::try_from(mods).context("Should be valid serialization")?,
        );

        let c = Result::and_then(toml::to_string_pretty(&tbl), |p| {
            Ok(p.replace("[[dependencies]]\n", ""))
        })
        .unwrap();

        let mut file = File::open(&cargo_path).unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .context("Cargo.toml file read error")?;

        if !buf.contains(MODULE_SEPARATOR) {
            buf.push_str(format!("\n{}", MODULE_SEPARATOR).as_str());
        }

        let mut final_buffer = String::new();

        for line in buf.lines() {
            final_buffer.push_str(format!("{}\n", line).as_str());
            if line == MODULE_SEPARATOR {
                break;
            }
        }

        if !is_empty {
            final_buffer.push_str(&format!("\n{c}"));
        }

        let out = std::env::var("OUT_DIR").unwrap();

        fs::write(cargo_path, final_buffer).unwrap();
        let p = Path::new(&out).join("module_reg.rs");
        let mut reg_file = File::create(p.clone())
            .inspect_err(|e| panic!("Failed to create file: {p:?}, got error: {e:?}"))
            .unwrap();
        for import in &module_imports {
            writeln!(reg_file, "{}", import).unwrap();
        }

        writeln!(
            reg_file,
            "pub fn register_modules(modules: &mut HashMap<String, Box<dyn Module>>) {{"
        )
        .unwrap();
        for registration in &module_reg {
            writeln!(reg_file, "    {}", registration).unwrap();
        }
        writeln!(reg_file, "}}").unwrap();

        Ok(())
    }
}

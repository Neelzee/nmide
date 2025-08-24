use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::{path::PathBuf, str::FromStr};
use tokio::process::Command;

#[derive(Debug)]
pub enum ModuleKind {
    RTRust,
    Rust,
    RTTypeScript { package_manager: String },
    TypeScript { package_manager: String },
    RTCss,
    Css,
    RTFile,
    File,
    Other(String),
}

pub struct Module {
    pub name: String,
    pub path: String,
    pub kind: ModuleKind,
    pub build_output: String,
}

impl Module {
    pub fn install(self) -> Result<()> {
        match self.kind {
            ModuleKind::RTRust => todo!(),
            ModuleKind::Rust => todo!(),
            ModuleKind::RTTypeScript { package_manager } => todo!(),
            ModuleKind::TypeScript { package_manager } => todo!(),
            ModuleKind::RTCss => todo!(),
            ModuleKind::Css => todo!(),
            ModuleKind::RTFile => todo!(),
            ModuleKind::File => todo!(),
            ModuleKind::Other(_) => todo!(),
        }
        Ok(())
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModuleConfigurationModel {
    package_manager: Option<String>,
    runtime: Option<RuntimeModuleConfigurationModel>,
    compile_time: Option<CompileTimeModuleConfigurationModel>,
}

impl ModuleConfigurationModel {
    pub fn to_modules(self) -> Result<Vec<Module>> {
        let mut modules = Vec::new();
        let mut pm = self.package_manager;
        if let Some(rt) = self.runtime {
            pm = if rt.package_manager.is_some() {
                rt.package_manager
            } else {
                pm
            };
            if rt.enabled.is_some_and(|b| b) {
                // TODO: Move this to own fn
                let pman = pm.unwrap();
                rt.typescript
                    .unwrap_or_default()
                    .into_iter()
                    .map(|decl| {
                        decl.module(
                            &mut modules,
                            ModuleKind::RTTypeScript {
                                package_manager: pman.clone(),
                            },
                        )
                    })
                    .collect::<Result<Vec<()>>>()?;
            } else {
                println!("Runtime modules disabled")
            }
        }
        Ok(modules)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CompileTimeModuleConfigurationModel {
    enabled: Option<bool>,
    package_manager: Option<String>,
    typescript: Option<Vec<ModuleDeclarationModel>>,
    rust: Option<Vec<ModuleDeclarationModel>>,
    css: Option<Vec<ModuleDeclarationModel>>,
    files: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RuntimeModuleConfigurationModel {
    enabled: Option<bool>,
    package_manager: Option<String>,
    typescript: Option<Vec<ModuleDeclarationModel>>,
    rust: Option<Vec<ModuleDeclarationModel>>,
    css: Option<Vec<ModuleDeclarationModel>>,
    files: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModuleDeclarationModel {
    path: String,
    build_output: Option<String>,
}

impl ModuleDeclarationModel {
    fn module(self, modules: &mut Vec<Module>, kind: ModuleKind) -> Result<()> {
        let path = PathBuf::from_str(&self.path)?
            .to_str()
            .ok_or(anyhow!("Could not get path: {}", self.path))?
            .to_string();
        let name = self
            .path
            .split_terminator("/")
            .last()
            .ok_or(anyhow!("Could not get name from path: {}", self.path))?
            .to_string();
        let build_output = self.build_output.unwrap_or(
            match &kind {
                ModuleKind::RTRust | ModuleKind::Rust => "target",
                ModuleKind::RTTypeScript { .. } | ModuleKind::TypeScript { .. } => "build",
                ModuleKind::RTCss => ".",
                ModuleKind::Css => ".",
                ModuleKind::RTFile => ".",
                ModuleKind::File => ".",
                ModuleKind::Other(path) => path,
            }
            .to_string(),
        );

        modules.push(Module {
            name,
            path,
            kind,
            build_output,
        });
        Ok(())
    }
}

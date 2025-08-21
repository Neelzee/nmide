//! Installs compile-time-modules
//!
//! Also responsible for cleanup.
//!
//! ## Installation
//!
//! Installation of modules vary, depending on the module kind. If it is an
//! JavaScript module, and a `package-manager` has been specified, the
//! cm-installer will run `package-manager i`, (npm/bun), installing all the
//! necessary dependencies, and then it will run `package-manager run build`,
//! running the build script for the module. It will then copy the resulting
//! file, expecting it to be `build/index.js`, to `dist/external`, and added
//! to the index.html file, as a script tag.
//!
//! If the module is a Rust compile-time-module, it will be directly added to
//! the Cargo.toml file as a dependency. The build-script for Nmide will ensure
//! it is properly installed, (i.e. that the module is actually "built" and
//! invoked).
//!
//! If the module is a Rust library, meaning a runtime-module, it will be built
//! and copied over to the `$APPDATA/modules` folder, where it will be picked up
//! by Nmide, during startup, and managed.
//!
//! If the module is a CSS module, it will be copied to `dist/external` and to
//! the index.html file.

use clean_up::clean_up;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use toml::Value;

mod clean_up;
mod css_installer;
mod js_installer;
mod js_rt_installer;
mod rs_installer;
mod rs_rt_installer;

const MODULE_SEP: &str = "<!--MODULES-->";

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy)]
pub(crate) enum Kind {
    #[default]
    Rust,
    JavaScript,
    MJavaScript,
    TypeScript,
    Css,
}

impl Kind {
    pub fn as_ext(&self) -> String {
        match self {
            Self::Rust => "rs".to_string(),
            Self::JavaScript => "js".to_string(),
            Self::TypeScript => "ts".to_string(),
            Self::MJavaScript => "mjs".to_string(),
            Self::Css => "css".to_string(),
        }
    }

    pub fn is_js(&self) -> bool {
        matches!(
            self,
            Kind::JavaScript | Kind::MJavaScript | Kind::TypeScript
        )
    }
}

impl From<String> for Kind {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "ts" => Self::TypeScript,
            "js" => Self::JavaScript,
            "mjs" => Self::MJavaScript,
            "css" => Self::Css,
            "rs" | "so" => Self::Rust,
            _ => panic!("Unknown extension: {value}"),
        }
    }
}

impl From<&str> for Kind {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Module {
    name: String,
    enabled: bool,
    path: PathBuf,
    kind: Kind,
    package_manager: Option<String>,
    features: Vec<String>,
}

impl Module {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn to_value(&self) -> Value {
        let mut mp = HashMap::new();
        mp.insert(
            "path",
            Value::String(self.path.to_str().unwrap().to_string()),
        );
        mp.insert(
            "features",
            Value::Array(self.features.clone().into_iter().map(Value::from).collect()),
        );
        Value::from(mp)
    }
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let mut conf = String::new();
    let mut cargo = String::new();
    let mut modules = String::new();
    let mut out = String::new();
    let mut dist = String::new();
    let mut index = String::new();
    let mut clean = false;
    let mut module_folder = String::new();
    let mut dry_run = false;
    args.for_each(|arg| {
        if arg.contains("--clean") {
            clean = true;
            return;
        }
        if arg.contains("--dry-run") {
            dry_run = true;
            return;
        }
        if arg.contains("--module-dist=") {
            module_folder = arg.replace("--module-dist=", "");
            return;
        }
        if arg.contains("--out=") {
            out = arg.replace("--out=", "");
            return;
        }
        if arg.contains("--index=") {
            index = arg.replace("--index=", "");
            return;
        }
        if arg.contains("--conf=") {
            conf = arg.replace("--conf=", "");
            return;
        }
        if arg.contains("--cargo=") {
            cargo = arg.replace("--cargo=", "");
            return;
        }
        if arg.contains("--modules=") {
            modules = arg.replace("--modules=", "");
            return;
        }
        if arg.contains("--dist=") {
            dist = arg.replace("--dist=", "");
        }
    });

    let modules_list = get_modules(&conf, &modules);

    let rt_modules = get_rt_modules(&conf, &modules);

    let files = get_files(&conf, &modules);

    if dry_run {
        if clean {
            println!(
                "Cleaning files: {:?}",
                vec![conf, cargo, modules, out, dist, index, module_folder]
            )
        } else {
            println!(
                "Installing modules to: {:?}",
                vec![
                    conf,
                    cargo,
                    modules,
                    out,
                    dist.clone(),
                    index,
                    module_folder
                ]
            );

            println!("\n{}\n", "=".repeat(80));
            println!("Installing modules:");
            println!(
                "{}",
                modules_list
                    .iter()
                    .map(|m| format!(
                        "Module: {}, path: {:?}, kind: {:?}",
                        m.name(),
                        m.path.to_str().unwrap(),
                        m.kind
                    ))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            println!("Copying files:");
            println!(
                "{}",
                files
                    .iter()
                    .map(|f| format!("File: {f} to {}", &dist))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
            println!("\n{}\n", "=".repeat(80));
        }

        return;
    }

    if clean {
        clean_up(index, cargo, out);
        return;
    }
    println!("\n{}\n", "=".repeat(80));
    println!("[modules]");
    println!(
        "{}",
        modules_list
            .iter()
            .filter(|m| m.enabled)
            .map(|m| format!(
                "Module: {}, path: {:?}, kind: {:?}",
                m.name(),
                m.path.to_str().unwrap(),
                m.kind
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
    println!("[rt-modules]");
    println!(
        "{}",
        rt_modules
            .iter()
            .filter(|m| m.enabled)
            .map(|m| format!(
                "Module: {}, path: {:?}, kind: {:?}",
                m.name(),
                m.path.to_str().unwrap(),
                m.kind
            ))
            .collect::<Vec<_>>()
            .join("\n")
    );
    println!("\n{}\n", "=".repeat(80));
    rs_installer::install(modules_list.clone(), cargo, out);
    rs_rt_installer::install(rt_modules.clone(), module_folder.clone());
    js_installer::install(dist.clone(), modules_list.clone());
    js_rt_installer::install(rt_modules.clone(), module_folder);
    let styles = css_installer::install(dist.clone(), modules_list);

    if index.is_empty() {
        println!("No index file to install");
        return;
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(&index)
        .unwrap_or_else(|err| panic!("Could not open file: {:?}, due to error: {:?}", index, err));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Could not read contents");
    let regex = Regex::new(r#"<!--MODULES-->(\s*|.*)*<!--MODULES-->"#).unwrap();
    let mut new_scripts = format!("{}\n", MODULE_SEP);
    new_scripts.push_str(styles.join("\n").as_str());
    new_scripts.push_str(format!("\n{}", MODULE_SEP).as_str());
    let new_content = regex.replace(&contents, new_scripts).to_string();
    file.write_at(new_content.as_bytes(), 0)
        .expect("Should be able to write to file");

    println!("Copying files:");
    for f in files {
        println!("{f}");
        let mut copy_cmd = Command::new("cp");
        copy_cmd.arg(f);
        copy_cmd.arg(format!("{}/", &dist));
        run_cmd(copy_cmd);
    }
}

fn get_files(conf: &str, modules: &str) -> Vec<String> {
    let mut files = Vec::new();
    let module_toml_path = Path::new(&conf)
        .canonicalize()
        .unwrap_or_else(|err| panic!("Can't canonicalize config: {modules:?}, error: {:?}", err));
    if !module_toml_path.exists() {
        panic!("Can't find files from {module_toml_path:?}");
    }
    let module_content = fs::read_to_string(&module_toml_path).unwrap_or_else(|_| {
        panic!("Path should exist, and be of valid encoding: {module_toml_path:?}")
    });
    let module_config: Value =
        toml::from_str(&module_content).expect("Module should be a valid TOML");

    let opt_files = module_config.get("files");
    if opt_files.is_none() {
        panic!("Can't find [files] section in {module_toml_path:?}");
    }
    let of = opt_files.unwrap().as_array();
    if of.is_none() {
        panic!("Files is not of valid type: {:?}", opt_files);
    }
    let opt_files = of.unwrap().clone();
    for file in opt_files {
        files.push(file.as_str().unwrap_or_default().to_string());
    }

    files
}

fn get_modules(conf: &str, modules: &str) -> Vec<Module> {
    let mut mods = Vec::new();
    let module_toml_path = Path::new(&conf)
        .canonicalize()
        .unwrap_or_else(|err| panic!("Can't canonicalize config: {modules:?}, error: {:?}", err));
    if !module_toml_path.exists() {
        panic!("Can't find modules from {module_toml_path:?}");
    }
    let module_content = fs::read_to_string(&module_toml_path).unwrap_or_else(|_| {
        panic!("Path should exist, and be of valid encoding: {module_toml_path:?}")
    });
    let module_config: Value =
        toml::from_str(&module_content).expect("Module should be a valid TOML");

    let opt_modules = module_config.get("modules").and_then(|p| p.as_table());
    if opt_modules.is_none() {
        panic!("Can't find [modules] section in {module_toml_path:?}");
    }
    let default_module_path = Path::new(&modules);
    for (module, spec) in opt_modules.unwrap() {
        let kind = spec
            .get("kind")
            .and_then(|v| v.as_str())
            .map(|s| -> Kind { s.into() })
            .unwrap_or_default();
        mods.push(Module {
            name: module.to_string(),
            enabled: spec
                .get("enabled")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            path: spec
                .get("path")
                .and_then(|v| v.as_str())
                .map(Path::new)
                .map(|p| {
                    p.canonicalize()
                        .unwrap_or_else(|_| panic!("Can't canonicalize path: {p:?}"))
                })
                .unwrap_or(default_module_path.join(format!("{}.{}", module, kind.as_ext()))),
            kind,
            package_manager: spec
                .get("package-manager")
                .and_then(|v| v.as_str())
                .map(|p| p.to_string()),
            features: spec
                .get("features")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or(Vec::new())
                .into_iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect(),
        })
    }

    mods
}

fn get_rt_modules(conf: &str, modules: &str) -> Vec<Module> {
    let mut mods = Vec::new();
    let module_toml_path = Path::new(&conf)
        .canonicalize()
        .unwrap_or_else(|err| panic!("Can't canonicalize config: {modules:?}, error: {:?}", err));
    if !module_toml_path.exists() {
        panic!("Can't find modules from {module_toml_path:?}");
    }
    let module_content = fs::read_to_string(&module_toml_path).unwrap_or_else(|_| {
        panic!("Path should exist, and be of valid encoding: {module_toml_path:?}")
    });
    let module_config: Value =
        toml::from_str(&module_content).expect("Module should be a valid TOML");

    let opt_modules = module_config.get("rt-modules").and_then(|p| p.as_table());
    if opt_modules.is_none() {
        panic!("Can't find [rt-modules] section in {module_toml_path:?}");
    }
    let default_module_path = Path::new(&modules);
    for (module, spec) in opt_modules.unwrap() {
        let kind = spec
            .get("kind")
            .and_then(|v| v.as_str())
            .map(|s| -> Kind { s.into() })
            .unwrap_or_default();
        mods.push(Module {
            name: module.to_string(),
            enabled: spec
                .get("enabled")
                .and_then(|v| v.as_bool())
                .unwrap_or(true),
            path: spec
                .get("path")
                .and_then(|v| v.as_str())
                .map(Path::new)
                .map(|p| {
                    p.canonicalize()
                        .unwrap_or_else(|_| panic!("Can't canonicalize path: {p:?}"))
                })
                .unwrap_or(default_module_path.join(format!("{}.{}", module, kind.as_ext()))),
            kind,
            package_manager: spec
                .get("package-manager")
                .and_then(|v| v.as_str())
                .map(|p| p.to_string()),
            features: spec
                .get("features")
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or(Vec::new())
                .into_iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect(),
        })
    }

    mods
}

pub(crate) fn run_cmd(mut cmd: Command) {
    match cmd.status() {
        Ok(st) if st.success() => (),
        Ok(st) => eprintln!(
            "Got non zero exit code! {:?},\
                 when running command: {:?} {:?}",
            st,
            cmd.get_program(),
            cmd.get_args(),
        ),
        Err(err) => eprintln!(
            "Failed running command: {:?} {:?}, got error: {:?}",
            cmd.get_program(),
            cmd.get_args(),
            err,
        ),
    }
}

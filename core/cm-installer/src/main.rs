use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use toml::Value;

mod rs_installer;
mod js_installer;

#[derive(Debug, Default, Eq, PartialEq, Clone)]
pub(crate) enum Kind {
    #[default]
    Rust,
    JavaScript,
    MJavaScript,
    TypeScript,
}

impl Kind {
    pub fn as_ext(&self) -> String {
        match self {
            Kind::Rust => "rs".to_string(),
            Kind::JavaScript => "js".to_string(),
            Kind::TypeScript => "ts".to_string(),
            Kind::MJavaScript => "mjs".to_string(),
        }
    }
}

impl From<String> for Kind {
    fn from(value: String) -> Self {
        match value.to_lowercase().as_str() {
            "ts" => Kind::TypeScript,
            "js" => Kind::JavaScript,
            "mjs" => Kind::MJavaScript,
            _ => Kind::Rust,
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
        mp.insert("path", Value::String(self.path.to_str().unwrap().to_string()));
        mp.insert("features", Value::Array(self.features.clone().into_iter().map(Value::from).collect()));
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
    args.for_each(|arg| {
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
            return;
        }
    });
    let modules = get_modules(conf, modules);
    //rs_installer::install(modules.clone(), cargo, out);
    js_installer::install(index, dist, modules);
}

fn get_modules(conf: String, modules: String) -> Vec<Module> {
    let mut mods = Vec::new();
    let module_toml_path = Path::new(&conf).canonicalize()
        .expect(&format!("Can't canonicalize config: {modules:?}"));
    if !module_toml_path.exists() {
        panic!("Can't find modules from {module_toml_path:?}");
    }
    let module_content = fs::read_to_string(&module_toml_path)
        .expect(format!("Path should exist, and be of valid encoding: {module_toml_path:?}").as_str());
    let module_config: Value =
        toml::from_str(&module_content).expect("Module should be a valid TOML");

    let opt_modules = module_config.get("modules").and_then(|p| p.as_table());
    if opt_modules.is_none() {
        panic!("Can't find [modules] section in {module_toml_path:?}");
    }
    let default_module_path = Path::new(&modules).canonicalize()
        .expect(&format!("Can't canonicalize config: {module_toml_path:?}"));
    for (module, spec) in opt_modules.unwrap() {
        let kind = spec.get("kind")
            .and_then(|v| v.as_str())
            .map(|s| -> Kind { s.into() })
            .unwrap_or_default();
        mods.push(
            Module {
                name: module.to_string(),
                enabled: spec.get("enabled")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(true),
                path: spec.get("path")
                    .and_then(|v| v.as_str())
                    .map(|s| Path::new(s))
                    .map(|p| p.canonicalize()
                            .expect(&format!("Can't canonicalize path: {p:?}")))
                    .unwrap_or(default_module_path.clone().join(format!("{}.{}", module.to_string(), kind.as_ext()))),
                kind,
                package_manager: spec.get("package-manager")
                    .and_then(|v| v.as_str())
                    .map(|p| p.to_string()),
                features: spec.get("features")
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or(Vec::new())
                    .into_iter()
                    .filter_map(|v| v.as_str().map(|s| s.to_string()))
                    .collect()
            }
        )
    }

    mods
}
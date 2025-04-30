use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::fs::OpenOptions;
use std::io::Read;
use std::os::unix::fs::FileExt;
use std::path::{Path, PathBuf};
use std::process::Command;
use toml::Value;

mod css_installer;
mod js_installer;
mod rs_installer;

const MODULE_SEP: &str = "<!--MODULES-->";

#[derive(Debug, Default, Eq, PartialEq, Clone)]
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
            _ => Self::Rust,
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
        }
    });
    let modules = get_modules(conf, modules);
    rs_installer::install(modules.clone(), cargo, out);
    let scripts = js_installer::install(dist.clone(), modules.clone());
    let styles = css_installer::install(dist, modules);

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
    new_scripts.push_str(scripts.join("\n").as_str());
    new_scripts.push('\n');
    new_scripts.push_str(styles.join("\n").as_str());
    new_scripts.push_str(format!("\n{}", MODULE_SEP).as_str());
    let new_content = regex.replace(&contents, new_scripts).to_string();
    file.write_at(new_content.as_bytes(), 0)
        .expect("Should be able to write to file");
}

fn get_modules(conf: String, modules: String) -> Vec<Module> {
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
    let default_module_path = Path::new(&modules)
        .canonicalize()
        .unwrap_or_else(|_| panic!("Can't canonicalize config: {module_toml_path:?}"));
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
                .unwrap_or(default_module_path.clone().join(format!(
                    "{}.{}",
                    module,
                    kind.as_ext()
                ))),
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

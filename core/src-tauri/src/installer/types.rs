use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toml::Value;

#[async_trait::async_trait]
pub trait ModuleInstaller {
    async fn install(&self, modules: &[(String, ModuleConfig)]) -> Result<()>;
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CoreModuleConfig {
    #[serde(default = "Vec::new")]
    pub files: Vec<String>,
    #[serde(default = "Vec::new")]
    pub rt_files: Vec<String>,
    #[serde(default = "HashMap::new")]
    pub modules: HashMap<String, ModuleConfig>,
    #[serde(default = "HashMap::new")]
    pub rt_modules: HashMap<String, ModuleConfig>,
}

fn default_kind() -> String {
    Kind::default().as_ext()
}

fn default_enable() -> bool {
    true
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ModuleConfig {
    #[serde(default = "default_enable")]
    pub(crate) enabled: bool,
    pub(crate) path: String,
    #[serde(default = "default_kind")]
    pub(crate) kind: String,
    pub(crate) package_manager: Option<String>,
    #[serde(default = "Vec::new")]
    pub(crate) features: Vec<String>,
}

impl ModuleConfig {
    pub fn to_value(&self) -> Value {
        let mut mp = HashMap::new();
        mp.insert("path", Value::String(self.path.clone()));
        mp.insert(
            "features",
            Value::Array(self.features.clone().into_iter().map(Value::from).collect()),
        );
        Value::from(mp)
    }
}

#[derive(Debug, Default, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
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

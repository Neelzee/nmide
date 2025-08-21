use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct CoreModuleConfig {
    pub package_manager: Option<String>,
    pub enabled: Option<bool>,
    #[serde(default = "Default::default")]
    pub compile_time: Config,
    #[serde(default = "Default::default")]
    pub runtime: Config,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    pub package_manager: Option<String>,
    pub enabled: Option<bool>,
    #[serde(default = "Vec::new")]
    pub files: Vec<String>,
    #[serde(default = "Vec::new")]
    pub typescript: Vec<ModuleConfig>,
    #[serde(default = "Vec::new")]
    pub javascript: Vec<ModuleConfig>,
    #[serde(default = "Vec::new")]
    pub rust: Vec<ModuleConfig>,
    #[serde(default = "Vec::new")]
    pub css: Vec<ModuleConfig>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ModuleConfig {
    path: Option<String>,
    url: Option<String>,
    version: Option<String>,
    package_manager: Option<String>,
    build_output: Option<String>,
}

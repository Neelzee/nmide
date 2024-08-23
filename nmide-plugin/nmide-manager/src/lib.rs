use std::{collections::HashMap, path::PathBuf, str::FromStr};

use anyhow::Result;
use nmide_plugin_manager::Nmlugin;
use nmide_std_lib::{
    attr::Attr,
    html::Html,
    map::{value::Value, Map},
    msg::Msg,
};
use once_cell::sync::Lazy;
use tokio::sync::Mutex;
use uuid::Uuid;

struct Plugin {
    id: Uuid,
    name: String,
    pub(crate) nmlugin: Nmlugin,
}

impl Plugin {
    pub fn new(id: Uuid, name: String, path: PathBuf) -> Result<Self> {
        Ok(Self {
            id,
            name,
            nmlugin: Nmlugin::new(path)?,
        })
    }
}

struct PluginManager {
    plugins_names: HashMap<String, Vec<Uuid>>,
    plugins: HashMap<Uuid, Plugin>,
    model: Map,
}

impl PluginManager {
    pub fn add_plugins(&mut self, plugins: Vec<(String, Uuid, PathBuf)>) {
        for p in plugins
            .into_iter()
            .filter_map(|(n, i, p)| Plugin::new(i, n, p).ok())
            .collect::<Vec<Plugin>>()
        {
            self.plugins_names.insert(p.name.clone(), vec![p.id]);
            self.plugins.insert(p.id, p);
        }
    }

    pub fn init(&mut self) -> Map {
        self.plugins
            .values()
            .into_iter()
            .fold(Map::new(), |model, p| {
                let plugin_model = p.nmlugin.init().unwrap_or_default();
                match plugin_model.lookup("nmide-plugin-framework") {
                    Some(Value::String(s)) if s == "true" => {
                        match self.model.lookup("nmide-plugin-framework") {
                            Some(Value::List(mut xs)) => {
                                xs.push(Value::String(p.id.to_string()));
                                self.model = self
                                    .model
                                    .clone()
                                    .insert("nmide-plugin-framework", Value::List(xs));
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }

                model.merge(plugin_model)
            })
    }

    pub fn view(&self, model: Map) -> Vec<Html> {
        self.plugins
            .values()
            .into_iter()
            .filter_map(|nl| nl.nmlugin.view(model.clone()).ok())
            .collect()
    }

    pub fn update(&self, msg: Msg, model: Map) -> Map {
        self.plugins.values().into_iter().fold(model, |m, p| {
            p.nmlugin.update(msg.clone(), m.clone()).unwrap_or(m)
        })
    }
}

static MANAGER: Lazy<Mutex<PluginManager>> = Lazy::new(|| {
    Mutex::new(PluginManager {
        plugins_names: HashMap::new(),
        plugins: HashMap::new(),
        model: Map::new(),
    })
});

static PLUGIN_PATH: Lazy<PathBuf> = Lazy::new(|| PathBuf::from("./plugin-libs/plugins"));

fn get_all_plugins() -> Result<Vec<(String, Uuid, PathBuf)>> {
    let path = PLUGIN_PATH.to_path_buf();
    let mut plugins = Vec::new();
    for entry in std::fs::read_dir(path)? {
        match entry {
            Ok(p) if p.path().is_file() => {
                if let Some(ext) = p.path().extension() {
                    if ext == "so" {
                        let uuid = Uuid::new_v4();
                        plugins.push((
                            p.file_name()
                                .to_str()
                                .unwrap_or(uuid.to_string().as_str())
                                .to_string(),
                            uuid,
                            p.path(),
                        ));
                    }
                }
            }
            _ => continue,
        }
    }
    Ok(plugins)
}

#[no_mangle]
pub extern "Rust" fn init() -> Map {
    let mut manager_lock = MANAGER.try_lock().unwrap();
    manager_lock.add_plugins(get_all_plugins().unwrap_or_default());
    manager_lock.init()
}

#[no_mangle]
pub extern "Rust" fn view(model: Map) -> Html {
    let manager_lock = MANAGER.try_lock().unwrap();
    Html::Frag {
        kids: manager_lock.view(model),
        attrs: Vec::new(),
    }
}

#[no_mangle]
pub extern "Rust" fn update(msg: Msg, model: Map) -> Map {
    let manager_lock = MANAGER.try_lock().unwrap();
    manager_lock.update(msg, model)
}

#[no_mangle]
pub extern "Rust" fn manifest() -> Map {
    let funcs: Map = vec![("nmide-functions", vec!["init", "view", "update"])].into();
    funcs.merge(vec![("nmide-plugin-type", "rust")].into())
}

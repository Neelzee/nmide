use std::{collections::HashMap, path::PathBuf, str::FromStr};

use anyhow::Result;
use nmide_plugin_manager::Nmlugin;
use nmide_rust_ffi::{
    attr::Attr,
    html::Html,
    map::Value,
    model::{Model, Msg},
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
    model: Model,
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

    pub fn init(&mut self) -> Model {
        self.plugins
            .values()
            .into_iter()
            .fold(Model::new(), |model, p| {
                let plugin_model = p.nmlugin.init().unwrap_or_default();
                match plugin_model.lookup("nmide-plugin-framework") {
                    Some(Value::Str(s)) if s == "true" => {
                        match self.model.lookup("nmide-plugin-framework") {
                            Some(Value::Arr(mut xs)) => {
                                xs.push(Value::Str(p.id.to_string()));
                                self.model = self
                                    .model
                                    .clone()
                                    .insert("nmide-plugin-framework", xs.into());
                            }
                            _ => (),
                        }
                    }
                    _ => (),
                }

                model.merge(plugin_model)
            })
    }

    pub fn view(&self, model: Model) -> Vec<Html> {
        self.plugins
            .values()
            .into_iter()
            .filter_map(|nl| nl.nmlugin.view(model.clone()).ok())
            .collect()
    }

    pub fn update(&self, msg: Msg, model: Model) -> Model {
        self.plugins.values().into_iter().fold(model, |m, p| {
            p.nmlugin.update(msg.clone(), m.clone()).unwrap_or(m)
        })
    }
}

static MANAGER: Lazy<Mutex<PluginManager>> = Lazy::new(|| {
    Mutex::new(PluginManager {
        plugins_names: HashMap::new(),
        plugins: HashMap::new(),
        model: Model::new(),
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
pub extern "Rust" fn init() -> Model {
    let mut manager_lock = MANAGER.try_lock().unwrap();
    manager_lock.add_plugins(get_all_plugins().unwrap_or_default());
    manager_lock.init()
}

#[no_mangle]
pub extern "Rust" fn view(model: Model) -> Html {
    let manager_lock = MANAGER.try_lock().unwrap();
    let frameworks = manager_lock
        .model
        .lookup("nmide-plugin-framework")
        .unwrap_or_default()
        .to_arr()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|v| v.to_str())
        .filter_map(|s| Uuid::from_str(&s).ok())
        .filter_map(|id| manager_lock.plugins.get(&id))
        .collect::<Vec<_>>();
    let frameworks_id = frameworks.iter().map(|pl| pl.id).collect::<Vec<_>>();
    let framework_html: Vec<Html> = frameworks
        .into_iter()
        .filter_map(|pl| pl.nmlugin.view(model.clone()).ok())
        .collect();
    let all_other_html = manager_lock
        .plugins
        .iter()
        .filter_map(|(k, v)| {
            if frameworks_id.contains(&k) {
                None
            } else {
                v.nmlugin.view(model.clone()).ok()
            }
        })
        .collect::<Vec<_>>();
    let mut frag = Html::Frag {
        kids: framework_html,
        attrs: Vec::new(),
    };
    for h in all_other_html {
        if let Some(location) = h.get_attr("location") {
            match location {
                Attr::Attr(_, loc) => frag.apply_if(
                    |html| match html.get_attr("id") {
                        Some(Attr::Id(ids))
                            if ids.split_whitespace().take_while(|k| k == &loc).count() > 0 =>
                        {
                            true
                        }
                        _ => false,
                    },
                    |html| html.adopt(h),
                ),
                _ => frag.adopt(h),
            }
        }
    }
    return frag;
}

#[no_mangle]
pub extern "Rust" fn update(msg: Msg, model: Model) -> Model {
    let manager_lock = MANAGER.try_lock().unwrap();
    manager_lock.update(msg, model)
}

#[no_mangle]
pub extern "Rust" fn manifest() -> Model {
    let funcs: Model = vec![("nmide-functions", vec!["init", "view", "update"])].into();
    funcs.merge(vec![("nmide-plugin-type", "rust")].into())
}

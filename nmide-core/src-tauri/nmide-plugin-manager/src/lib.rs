use abi_stable::library::{LibraryPath, RootModule};
use anyhow::Result;
use nmide_std_lib::{
    html::rhtml::RHtml, map::rmap::RMap, msg::rmsg::RMsg, NmideStandardLibrary_Ref,
};
use std::path::Path;

pub struct Nmlugin {
    lib: NmideStandardLibrary_Ref,
    plugin_path: String,
}

impl Nmlugin {
    pub fn new(path: &Path) -> Result<Self> {
        let pp = LibraryPath::FullPath(path);
        //let plugin = NmideStandardLibrary_Ref::load_from_directory(&plugin_path)?;
        let plugin = NmideStandardLibrary_Ref::load_from(pp)?;

        Ok(Self {
            lib: plugin,
            plugin_path: path.to_string_lossy().to_string(),
        })
    }

    pub fn init(&self) -> RMap {
        self.lib.init()()
    }

    pub fn view(&self, model: RMap) -> RHtml {
        self.lib.view()(model)
    }

    pub fn update(&self, msg: RMsg, model: RMap) -> RMap {
        self.lib.update()(msg, model)
    }
}

impl std::fmt::Debug for Nmlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Nmlugin")
            .field("plugin_path", &self.plugin_path)
            .finish()
    }
}

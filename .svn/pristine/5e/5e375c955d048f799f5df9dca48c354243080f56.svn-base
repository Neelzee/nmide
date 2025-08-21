#![warn(
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]

use abi_stable::library::{LibraryError, LibraryPath, RootModule};
use core_std_lib::{html::rhtml::RHtml, map::rmap::RMap, msg::rmsg::RMsg, NmideStandardLibraryRef};
use std::path::Path;

pub struct Nmlugin {
    lib: NmideStandardLibraryRef,
    plugin_path: String,
}

impl Nmlugin {
    pub fn new(path: &Path) -> Result<Self, LibraryError> {
        Ok(Self {
            lib: NmideStandardLibraryRef::load_from(LibraryPath::FullPath(path))?,
            plugin_path: path.to_string_lossy().to_string(),
        })
    }

    pub fn init(&self) -> RMap {
        self.lib.init()()
    }

    pub fn view(&self, model: &RMap) -> RHtml {
        self.lib.view()(model)
    }

    pub fn update(&self, msg: &RMsg, model: &RMap) -> RMap {
        self.lib.update()(msg, model)
    }

    pub fn name(&self) -> &str {
        self.plugin_path.split("/").last().unwrap_or_default()
    }

    pub fn path(&self) -> &str {
        &self.plugin_path
    }
}

impl std::fmt::Debug for Nmlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Nmlugin")
            .field("plugin_path", &self.plugin_path)
            .finish()
    }
}

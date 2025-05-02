use std::{future::Future, path::Path, process::Output};
use abi_stable::{
    library::{LibraryError, LibraryPath, RootModule}, package_version_strings, sabi_trait, sabi_types::VersionStrings, std_types::{self, RArc, RBox, ROption, RString}, StableAbi
};
use foreign_std_lib::{core::rs_core_modification::RCoreModification, event::rs_event::REvent, html::rs_html::RHtml, state::rs_state::RState};

use crate::Module;

#[sabi_trait]
pub trait RCore: Send + Sync {
    async extern "C" fn state(&self) -> RState;
    async extern "C" fn ui(&self) -> RHtml;
    async extern "C" fn throw_event(&self, event: REvent);
    async extern "C" fn add_handler(
        &self,
        event_name: ROption<RString> ,
        module_name: ROption<RString>,
        handler_name: RString,
    );
    async extern "C" fn send_modification(&self, modification: RCoreModification);
}

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = ModuleRef)))]
#[sabi(missing_field(panic))]
pub struct RustModule {
    pub init: extern "C" fn(core: RCore_CTO),
    pub handler: extern "C" fn(event: REvent, core: RCore_CTO),
}

impl RootModule for ModuleRef {
    abi_stable::declare_root_module_statics! {ModuleRef}

    const BASE_NAME: &'static str = "RS_COMP_MOD";
    const NAME: &'static str = "NMIDE";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

pub fn load_root_module_in_directory(directory: &Path) -> Result<ModuleRef, LibraryError> {
    ModuleRef::load_from_directory(directory)
}


pub struct RsModule {
    module: ModuleRef,
    module_path: String,
}

impl RsModule {
    pub fn new(path: &Path) -> Result<Self, LibraryError> {
        Ok(Self {
            module: ModuleRef::load_from(LibraryPath::FullPath(path))?,
            module_path: path.to_string_lossy().to_string(),
        })
    }

    pub fn name(&self) -> &str {
        &self.module_path
    }

    pub fn init(&self, core: RCore_CTO) {
        self.module.init()(core);
    }

    pub fn path(&self) -> &str {
        &self.module_path
    }
}
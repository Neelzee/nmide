#![allow(non_local_definitions)]
use abi_stable::{
    library::{LibraryError, LibraryPath, RootModule},
    package_version_strings, sabi_trait,
    sabi_types::VersionStrings,
    std_types::RString,
    StableAbi,
};
use async_ffi::FfiFuture;
use foreign_std_lib::{
    core::rs_core_modification::RCoreModification, event::rs_event::REvent, html::rs_html::RHtml,
    state::rs_state::RState,
};
use std::{fmt::Debug, future::IntoFuture, path::Path};

#[sabi_trait]
pub trait RCore: Send + Sync {
    /// Retrieves the current state
    async extern "C" fn state(&self) -> FfiFuture<RState>;

    /// Retrieves the current UI representation
    async extern "C" fn ui(&self) -> FfiFuture<RHtml>;

    async extern "C" fn throw_event(&self, event: REvent) -> FfiFuture<()>;

    /// Maps an Event to the given handler (Module name)
    async extern "C" fn add_handler(
        &self,
        event_name: RString,
        handler_name: RString,
    ) -> FfiFuture<()>;
    async extern "C" fn send_modification(&self, modification: RCoreModification) -> FfiFuture<()>;
}

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = ModuleRef)))]
#[sabi(missing_field(panic))]
pub struct RustModule {
    pub init: extern "C" fn(core: RCore_CTO<'static, 'static>) -> FfiFuture<()>,
    pub handler: extern "C" fn(event: REvent, core: RCore_CTO<'static, 'static>) -> FfiFuture<()>,
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

    pub fn path(&self) -> &str {
        &self.module_path
    }

    pub fn name(&self) -> String {
        let pth = self.path();
        pth.split("/").last().unwrap_or(pth).to_string()
    }

    pub async fn init(&self, core: RCore_CTO<'static, 'static>) {
        async move { self.module.init()(core).await }
            .into_future()
            .await;
    }

    pub async fn handler(&self, event: REvent, core: RCore_CTO<'static, 'static>) {
        async move { self.module.handler()(event, core).await }
            .into_future()
            .await;
    }
}

impl Debug for RsModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RsModule")
            .field("module_path", &self.module_path)
            .finish()
    }
}

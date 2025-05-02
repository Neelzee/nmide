use std::path::Path;

use abi_stable::{
    library::{LibraryError, RootModule}, package_version_strings, sabi_types::VersionStrings, std_types::RBox, StableAbi
};

use crate::{core::rs_core::RCore, event::rs_event::REvent};

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = ModuleRef)))]
#[sabi(missing_field(panic))]
pub struct RustModule {
    pub init: extern "C" fn(core: RBox<dyn RCore>) -> (),
    pub handler: extern "C" fn(event: REvent, core: u8) -> (),
}

impl RootModule for ModuleRef {
    abi_stable::declare_root_module_statics! {ModuleRef}

    const BASE_NAME: &'static str = "module_example";
    const NAME: &'static str = "module_example";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

pub fn load_root_module_in_directory(directory: &Path) -> Result<ModuleRef, LibraryError> {
    ModuleRef::load_from_directory(directory)
}

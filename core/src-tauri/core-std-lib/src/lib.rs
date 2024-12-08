pub mod attr;
pub mod html;
pub mod map;
pub mod msg;

use std::path::Path;

use abi_stable::{
    library::{LibraryError, RootModule},
    package_version_strings,
    sabi_types::VersionStrings,
    StableAbi,
};
use html::rhtml::RHtml;
use map::rmap::RMap;
use msg::rmsg::RMsg;

#[repr(C)]
#[derive(StableAbi)]
#[sabi(kind(Prefix(prefix_ref = NmideStandardLibraryRef)))]
#[sabi(missing_field(panic))]
pub struct NmideStdLib {
    pub init: extern "C" fn() -> RMap,
    pub view: extern "C" fn(model: &RMap) -> RHtml,
    pub update: extern "C" fn(msg: &RMsg, model: &RMap) -> RMap,
}

impl RootModule for NmideStandardLibraryRef {
    abi_stable::declare_root_module_statics! {NmideStandardLibraryRef}

    const BASE_NAME: &'static str = "nmide_example_plugin";
    const NAME: &'static str = "nmide_example_plugin";
    const VERSION_STRINGS: VersionStrings = package_version_strings!();
}

pub fn load_root_module_in_directory(
    directory: &Path,
) -> Result<NmideStandardLibraryRef, LibraryError> {
    NmideStandardLibraryRef::load_from_directory(directory)
}

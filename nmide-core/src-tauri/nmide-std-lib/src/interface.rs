pub mod rfunctions {
    use crate::{html::Html, map::Map, msg::Msg};

    pub type RInit = unsafe extern "Rust" fn() -> Map;

    pub type RUpdate = unsafe extern "Rust" fn(Msg, Map) -> Map;

    pub type RView = unsafe extern "Rust" fn(Map) -> Html;

    pub type RManifest = unsafe extern "Rust" fn() -> Map;
}

pub mod cfunctions {
    use crate::{CHtml, CModel, CMsg};

    pub type CInit = unsafe extern "C" fn() -> CModel;

    pub type CUpdate = unsafe extern "C" fn(CMsg, CModel) -> CModel;

    pub type CView = unsafe extern "C" fn(CModel) -> CHtml;

    pub type CManifest = unsafe extern "C" fn() -> CModel;
}

pub mod rfunctions {
    use crate::{
        html::Html,
        model::{Model, Msg},
    };

    pub type RInit = unsafe extern "Rust" fn() -> Model;

    pub type RUpdate = unsafe extern "Rust" fn(Msg, Model) -> Model;

    pub type RView = unsafe extern "Rust" fn(Model) -> Html;

    pub type RManifest = unsafe extern "Rust" fn() -> Model;
}

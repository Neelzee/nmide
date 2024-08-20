pub mod cfunctions {
    use crate::{CHtml, CModel, CMsg};

    pub type CInit = unsafe extern "C" fn() -> CModel;

    pub type CUpdate = unsafe extern "C" fn(CMsg, CModel) -> CModel;

    pub type CView = unsafe extern "C" fn(CModel) -> CHtml;

    pub type CManifest = unsafe extern "C" fn() -> CModel;
}

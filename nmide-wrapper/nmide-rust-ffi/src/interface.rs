pub mod functions {

    use crate::{CHtml, CModel, CMsg};

    pub type Init = unsafe extern "C" fn() -> CModel;

    pub type Update = unsafe extern "C" fn(CMsg, CModel) -> CModel;

    pub type View = unsafe extern "C" fn(CModel) -> CHtml;
}

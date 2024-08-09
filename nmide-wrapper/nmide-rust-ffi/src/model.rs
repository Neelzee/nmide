use anyhow::Result;

use crate::{CModel, CMsg};

pub struct Model;

pub enum Msg {}

impl Msg {
    pub fn from_c(cmsg: CMsg) -> Result<Self> {
        todo!()
    }

    pub fn to_c(self) -> Result<CMsg> {
        todo!()
    }
}

impl Model {
    pub fn from_c(cmodel: CModel) -> Result<Self> {
        todo!()
    }

    pub fn to_c(self) -> Result<CModel> {
        todo!()
    }
}

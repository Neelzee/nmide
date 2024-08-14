use std::{
    ffi::{CStr, CString},
    ptr::null_mut,
};

use anyhow::Result;
use safer_ffi::prelude::AsOut;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    map::{Map, Value},
    CModel, CMsg, CVal, CValType_Str, CValUnion, MaybeVal,
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Model {
    map: Map,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub enum Msg {
    PluginMsg(String),
}

impl Msg {
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    ///
    /// # Safety
    ///
    /// .
    pub unsafe fn from_c(cmsg: CMsg) -> Result<Self> {
        Ok(Self::PluginMsg(
            CStr::from_ptr(cmsg.msg).to_string_lossy().to_string(),
        ))
    }

    pub fn to_c(self) -> Result<CMsg> {
        match self {
            Msg::PluginMsg(s) => {
                let len = s.chars().count();
                Ok(CMsg {
                    msg: CString::new(s)?.as_ptr().cast_mut(),
                    len,
                    opt: MaybeVal {
                        just: false,
                        val: CVal {
                            type_: CValType_Str,
                            val: CValUnion {
                                str_: CString::new("")?.as_ptr(),
                            },
                        },
                    },
                })
            }
        }
    }
}

impl Model {
    pub fn new() -> Self {
        Self { map: Map::new() }
    }

    pub fn merge(mut self, mut other: Self) -> Self {
        self.map.map.append(&mut other.map.map);
        Self { map: self.map }
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    ///
    /// # Safety
    ///
    /// .
    pub unsafe fn from_c(cmodel: CModel) -> Result<Self> {
        Ok(Self {
            map: Map::from_c(cmodel.map)?,
        })
    }

    pub fn to_c(self) -> Result<CModel> {
        Ok(CModel {
            map: Map::to_c(self.map)?,
        })
    }
}

impl<T> From<Vec<(String, T)>> for Model
where
    T: Into<Value>,
{
    fn from(value: Vec<(String, T)>) -> Self {
        Self { map: value.into() }
    }
}

impl<T> From<Vec<(&str, T)>> for Model
where
    T: Into<Value>,
{
    fn from(value: Vec<(&str, T)>) -> Self {
        Self { map: value.into() }
    }
}

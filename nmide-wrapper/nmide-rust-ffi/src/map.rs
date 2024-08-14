use std::ffi::{CStr, CString};

use anyhow::{Context, Result};
use safer_ffi::prelude::AsOut;
use serde::{Deserialize, Serialize};

use crate::{
    CKeyPair, CMap, CVal, CValType_Arr, CValType_Int, CValType_Obj, CValType_Str, CValUnion,
};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Map {
    pub(crate) map: Vec<(String, Value)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    Int(i32),
    Str(String),
    Arr(Vec<Value>),
    Obj(Map),
}

impl Value {
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    ///
    /// # Safety
    ///
    /// .
    pub unsafe fn from_c(cval: CVal) -> Result<Self> {
        match cval.type_ {
            CValType_Int => Ok(Self::Int(
                cval.val._int, // unsafe
            )),
            CValType_Str => Ok(Self::Str(
                CStr::from_ptr(
                    cval.val.str_, // unsafe
                ) // unsafe
                .to_str()?
                .to_string(),
            )),
            CValType_Arr => todo!(),
            CValType_Obj => todo!(),
            _ => unimplemented!(),
        }
    }

    pub fn to_c(self) -> Result<CVal> {
        match self {
            Value::Int(v) => Ok(CVal {
                type_: CValType_Int,
                val: CValUnion { _int: v },
            }),
            Value::Str(v) => Ok(CVal {
                type_: CValType_Str,
                val: CValUnion {
                    str_: CString::new(v)?.as_c_str().as_ptr(),
                },
            }),
            Value::Arr(_v) => todo!(),
            Value::Obj(_v) => todo!(),
        }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::Str(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::Str(value.to_string())
    }
}

impl From<i32> for Value {
    fn from(value: i32) -> Self {
        Self::Int(value)
    }
}

impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(value: Vec<T>) -> Self {
        Self::Arr(value.into_iter().map(Into::into).collect::<Vec<Value>>())
    }
}

impl Map {
    pub fn new() -> Self {
        Self { map: Vec::new() }
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
    pub unsafe fn from_c(cmap: CMap) -> Result<Self> {
        let vec: Result<Vec<(String, Value)>> =
            Vec::from_raw_parts(cmap.values, cmap.len, std::mem::size_of::<*mut CKeyPair>())
                .into_iter()
                .map(|ck| from_ckey_pair(ck))
                .collect();
        Ok(Self { map: vec? })
    }

    pub fn to_c(self) -> Result<CMap> {
        let len = self.map.len();
        Ok(CMap {
            values: self
                .map
                .into_iter()
                .map(to_ckey_pair)
                .collect::<Result<Vec<_>>>()?
                .as_mut_ptr(),
            len,
        })
    }
}

impl<T> From<Vec<(String, T)>> for Map
where
    T: Into<Value>,
{
    fn from(value: Vec<(String, T)>) -> Self {
        Self {
            map: value.into_iter().map(|(k, v)| (k, v.into())).collect(),
        }
    }
}

impl<T> From<Vec<(&str, T)>> for Map
where
    T: Into<Value>,
{
    fn from(value: Vec<(&str, T)>) -> Self {
        Self {
            map: value
                .into_iter()
                .map(|(k, v)| (k.to_string(), v.into()))
                .collect(),
        }
    }
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
pub unsafe fn from_ckey_pair(ck: CKeyPair) -> Result<(String, Value)> {
    Ok((
        CStr::from_ptr(ck.key).to_str()?.to_string(),
        Value::from_c(ck.val)?,
    ))
}

pub fn to_ckey_pair(keypair: (String, Value)) -> Result<CKeyPair> {
    Ok(CKeyPair {
        key: CString::new(keypair.0)?.as_c_str().as_ptr(),
        val: keypair.1.to_c()?,
    })
}

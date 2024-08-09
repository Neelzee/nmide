use anyhow::{anyhow, Context, Result};

use crate::{
    html::from_char, CMap, CVal, CValType, CValType_Arr, CValType_Int, CValType_Obj, CValType_Str,
};

pub struct Map {
    map: Vec<(String, Value)>,
}

pub enum Value {
    Int(i32),
    Str(String),
    Arr(Vec<Value>),
    Obj(Map),
}

impl Value {
    pub fn from_c(cval_ptr: *mut CVal) -> Result<Self> {
        if let Some(cval) = unsafe { cval_ptr.as_ref() } {
            match (
                cval.type_,
                unsafe { cval.val.as_ref() }.with_context(|| "CValUnion should not be NULL")?,
            ) {
                (CValType_Int, val) => Ok(Self::Int(unsafe { val._int })),
                (CValType_Str, val) => Ok(Self::Str(from_char(unsafe { val.str_ })?)),
                (CValType_Arr, _) => unimplemented!(),
                (CValType_Obj, _) => unimplemented!(),
                _ => unreachable!("CValType should not be: {}", cval.type_),
            }
        } else {
            Err(anyhow!("CVal is NULL"))
        }
    }
}

impl Map {
    pub fn from_c(cmap_ptr: *mut CMap) -> Result<Self> {
        let cmap = unsafe { cmap_ptr.as_ref() }.with_context(|| "CMap is NULL")?;
        let values = unsafe { cmap.values.read() };
        todo!()
    }
}

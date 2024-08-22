use std::{borrow::Borrow, collections::VecDeque, ffi::CString, ptr::null_mut};

use crate::{util::from_char, CHtml, CHtmlContent, CHtmlElement, CHtmlTag_Div, CHtmlTag_None};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

impl Default for CHtml {
    fn default() -> Self {
        Self {
            content: Default::default(),
            isElement: false,
        }
    }
}

impl Default for CHtmlContent {
    fn default() -> Self {
        Self {
            text: CString::new("").unwrap_or_default().as_ptr(),
        }
    }
}

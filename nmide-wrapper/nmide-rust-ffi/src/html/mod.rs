use core::str;
use std::ffi::{CStr, CString};

use anyhow::{Context, Result};
use c_vec::CVec;
use safer_ffi::c_char;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    CHtml, CHtmlContent, CHtmlElement, CHtmlTag, CHtmlTag_A, CHtmlTag_Aside, CHtmlTag_Button, CHtmlTag_Div, CHtmlTag_Input, CHtmlTag_Nav, CHtmlTag_None, CHtmlTag_P, CHtmlTag_Script, CHtmlTag_Section, CHtmlTag_Select, CHtmlTag_Span
};

#[cfg(test)]
mod tests;

#[derive(Debug, TS, Serialize, Deserialize, PartialEq, Eq)]
#[ts(export)]
pub enum Element {
    Div,
    P,
    Span,
    Section,
    Input,
    Button,
    Text(String),
    Script,
    Select,
    Aside,
    Nav,
    A,
    None,
}

impl Element {
    pub fn from_c(ce: CHtmlTag) -> Self {
        match ce {
            CHtmlTag_Div => Self::Div,
            CHtmlTag_P => Self::P,
            CHtmlTag_Span => Self::Span,
            CHtmlTag_Section => Self::Section,
            CHtmlTag_Input => Self::Input,
            CHtmlTag_Button => Self::Button,
            CHtmlTag_Script => Self::Script,
            CHtmlTag_Select => Self::Select,
            CHtmlTag_Aside => Self::Aside,
            CHtmlTag_Nav => Self::Nav,
            CHtmlTag_A => Self::A,
            _ => Self::None,
        }
    }

    pub fn to_c(self) -> CHtmlTag {
        match self {
            Element::Div => CHtmlTag_Div,
            Element::P => CHtmlTag_P,
            Element::Span => CHtmlTag_Span,
            Element::Section => CHtmlTag_Section,
            Element::Input => CHtmlTag_Input,
            Element::Button => CHtmlTag_Button,
            Element::Script => CHtmlTag_Script,
            Element::Select => CHtmlTag_Select,
            Element::Aside => CHtmlTag_Aside,
            Element::Nav => CHtmlTag_Nav,
            Element::A => CHtmlTag_A,
            _ => CHtmlTag_None,
        }
    }
}

#[derive(Debug, TS, Serialize, Deserialize, PartialEq, Eq)]
#[ts(export)]
pub struct Html {
    kind: Element,
    kids: Vec<Html>,
}

#[derive(Debug, TS, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[ts(export)]
pub enum RHtml {
    Div { kids: Vec<RHtml> },
    Text(String),
}

impl RHtml {
    pub fn from_c(chtml: CHtml) -> Result<Self> {
        if chtml.isElement {
            let element: CHtmlElement = unsafe { *(*chtml.content).element };

            let raw_kids = element.children;

            let mut kids = Vec::with_capacity(element.len);

            for i in 0..element.len {
                let ptr = unsafe { *raw_kids.add(i) };

                if !ptr.is_null() {
                    kids.push(Self::from_c(unsafe { *ptr })?);
                }

            }

            match Element::from_c(element.tag) {
                Element::Div => Ok(Self::Div { kids }),
                _ => unimplemented!("TODO"),
            }

        } else {
            Ok(Self::Text(from_char(unsafe { (*(*chtml.content).text).text.clone() })?))
        }
    }
}

pub fn from_char(c: *mut i8) -> Result<String> {
    Ok(unsafe {CStr::from_ptr(c)}.to_str()?.to_string())
}

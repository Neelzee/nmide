use core::str;
use std::{
    ffi::{CStr, CString},
    ptr::{null, null_mut},
};

use anyhow::{anyhow, Context, Result};
use safer_ffi::{c_char, prelude::AsOut};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    CHtml, CHtmlContent, CHtmlElement, CHtmlLocation, CHtmlTag, CHtmlTag_A, CHtmlTag_Aside,
    CHtmlTag_Button, CHtmlTag_Div, CHtmlTag_Input, CHtmlTag_Nav, CHtmlTag_None, CHtmlTag_P,
    CHtmlTag_Script, CHtmlTag_Section, CHtmlTag_Select, CHtmlTag_Span, CHtmlText,
};

#[cfg(test)]
mod tests;

impl From<CHtml> for Html {
    fn from(value: CHtml) -> Self {
        Self::from_c(value).unwrap_or_default()
    }
}

impl From<Html> for CHtml {
    fn from(value: Html) -> Self {
        value.to_c().unwrap_or_default()
    }
}

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

pub fn chtml_location(cl_ptr: *mut CHtmlLocation) -> Result<(String, Html)> {
    let cl = unsafe { cl_ptr.as_ref() }.with_context(|| "CHtmlLocation is NULL")?;
    let loc = from_char(cl.location)?;
    let html = Html::from_c(cl.html);
    Ok((loc, html?))
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

#[derive(Debug, TS, Serialize, Deserialize, PartialEq, Eq, Clone)]
#[ts(export)]
pub enum Html {
    Div { kids: Vec<Html> },
    Text(String),
    None,
}

impl Html {
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
            Ok(Self::Text(from_char(unsafe {
                (*(*chtml.content).text).text.clone()
            })?))
        }
    }
    pub fn to_c(self) -> Result<CHtml> {
        match self {
            Html::Text(s) => Ok(CHtml {
                content: CHtmlContent {
                    text: CHtmlText {
                        text: to_char(&s)?,
                        len: s.chars().count(),
                    }
                    .as_out()
                    .as_mut_ptr(),
                }
                .as_out()
                .as_mut_ptr(),
                isElement: false,
            }),
            Html::Div { kids } => {
                let len = kids.len();
                Ok(CHtml {
                    content: CHtmlContent {
                        element: CHtmlElement {
                            tag: CHtmlTag_Div,
                            children: kids
                                .into_iter()
                                .map(|k| k.to_c())
                                .filter(|k| k.is_ok())
                                .map(|k| k.unwrap())
                                .collect::<Vec<CHtml>>()
                                .as_out()
                                .as_mut_ptr()
                                .as_out()
                                .as_mut_ptr(),
                            len,
                        }
                        .as_out()
                        .as_mut_ptr(),
                    }
                    .as_out()
                    .as_mut_ptr(),
                    isElement: true,
                })
            }
            _ => Err(anyhow!("Can't convert `{:?}`", self)),
        }
    }
}

impl Default for Html {
    fn default() -> Self {
        Self::None
    }
}

impl Default for CHtml {
    fn default() -> Self {
        CHtml {
            content: CHtmlContent {
                element: CHtmlElement {
                    tag: CHtmlTag_None,
                    children: null_mut(),
                    len: 0,
                }
                .as_out()
                .as_mut_ptr(),
            }
            .as_out()
            .as_mut_ptr(),
            isElement: true,
        }
    }
}

pub fn from_char(c: *mut i8) -> Result<String> {
    Ok(unsafe { CStr::from_ptr(c) }.to_str()?.to_string())
}

pub fn to_char(s: &str) -> Result<*mut i8> {
    Ok(CString::new(s)?
        .into_bytes_with_nul()
        .into_iter()
        .map(|c| c as i8)
        .collect::<Vec<i8>>()
        .as_mut_ptr())
}

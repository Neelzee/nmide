use std::{ffi::CString, ptr::null_mut};

use crate::{
    attr::Attr, util::from_char, CHtml, CHtmlContent, CHtmlElement, CHtmlTag_Div, CHtmlTag_None,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub enum Html {
    Div { kids: Vec<Html>, attrs: Vec<Attr> },
    Btn { kids: Vec<Html>, attrs: Vec<Attr> },
    Text(String),
    None,
}

impl Html {
    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    ///
    /// # Safety
    ///
    /// .
    pub unsafe fn from_c(ch: CHtml) -> Result<Self> {
        let content = ch.content;

        if !ch.isElement {
            return Ok(Self::Text(from_char(
                content.text, // unsafe
            )?));
        }
        let element = content.element;
        match element.tag {
            CHtmlTag_Div => Ok(Self::Div {
                kids: Vec::from_raw_parts(
                    // unsafe
                    element.children,
                    element.len,
                    std::mem::size_of::<CHtml>(),
                )
                .into_iter()
                .filter_map(|ptr| {
                    Self::from_c(ptr).ok() // unsafe
                })
                .collect(),

                attrs: Vec::new(),
            }),
            _ => Ok(Self::None),
        }
    }

    pub fn to_c(self) -> Result<CHtml> {
        match self {
            Html::Div { kids, .. } => {
                let len = kids.len();
                Ok(CHtml {
                    content: CHtmlContent {
                        element: CHtmlElement {
                            tag: CHtmlTag_Div,
                            children: kids
                                .into_iter()
                                .filter_map(|k| Self::to_c(k).ok())
                                .collect::<Vec<_>>()
                                .as_mut_ptr(),
                            len,
                        },
                    },
                    isElement: true,
                })
            }
            Html::Text(s) => Ok(CHtml {
                content: CHtmlContent {
                    text: CString::new(s.clone())?.as_c_str().as_ptr(),
                },
                isElement: false,
            }),
            _ => Ok(CHtml {
                content: CHtmlContent {
                    element: CHtmlElement {
                        tag: CHtmlTag_None,
                        children: null_mut(),
                        len: 0,
                    },
                },
                isElement: true,
            }),
        }
    }

    pub fn get_text(&self) -> String {
        match self {
            Html::Div { kids, .. } => kids.iter().map(Self::get_text).collect::<String>(),
            Html::Btn { kids, .. } => kids.iter().map(Self::get_text).collect::<String>(),
            Html::Text(s) => s.clone(),
            Html::None => String::new(),
        }
    }
}

#[test]
fn to_and_from_html() -> Result<()> {
    let html = Html::Text("Hello, World!".to_string());

    let chtml = html.to_c()?;

    let res = unsafe { Html::from_c(chtml) };
    assert!(res.is_ok());
    assert_eq!(res.unwrap().get_text(), "Hello, World!");

    Ok(())
}

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

use core::str;

use anyhow::{Context, Result};
use c_vec::CVec;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    CElement, CElement_A, CElement_Aside, CElement_Button, CElement_Div, CElement_Input,
    CElement_Nav, CElement_None, CElement_P, CElement_Script, CElement_Section, CElement_Select,
    CElement_Span, CHtml, CHtmlText, CHtmlUnion,
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
    pub fn from_c(ce: CElement) -> Self {
        match ce {
            CElement_Div => Self::Div,
            CElement_P => Self::P,
            CElement_Span => Self::Span,
            CElement_Section => Self::Section,
            CElement_Input => Self::Input,
            CElement_Button => Self::Button,
            CElement_Script => Self::Script,
            CElement_Select => Self::Select,
            CElement_Aside => Self::Aside,
            CElement_Nav => Self::Nav,
            CElement_A => Self::A,
            _ => Self::None,
        }
    }

    pub fn to_c(self) -> CElement {
        match self {
            Element::Div => CElement_Div,
            Element::P => CElement_P,
            Element::Span => CElement_Span,
            Element::Section => CElement_Section,
            Element::Input => CElement_Input,
            Element::Button => CElement_Button,
            Element::Script => CElement_Script,
            Element::Select => CElement_Select,
            Element::Aside => CElement_Aside,
            Element::Nav => CElement_Nav,
            Element::A => CElement_A,
            _ => CElement_None,
        }
    }
}

#[derive(Debug, TS, Serialize, Deserialize, PartialEq, Eq)]
#[ts(export)]
pub struct Html {
    kind: Element,
    kids: Vec<Html>,
}

impl Html {
    pub fn new(kind: Element, kids: Vec<Self>) -> Self {
        Self { kind, kids }
    }

    pub fn from_c(ch: CHtml) -> Result<Self> {
        if ch.isNode == 0 {
            return Ok(Self::new(
                Element::Text(unsafe { Self::c_str(ch.node.text)? }),
                Vec::new(),
            ));
        }

        let kind = unsafe { ch.node.kind };

        let cvec: CVec<CHtml> = unsafe { CVec::new(ch.kids, ch.kid_count as usize) };

        let mut kids = Vec::new();

        for sch in cvec.iter() {
            kids.push(Self::from_c(sch.to_owned())?);
        }

        Ok(Self {
            kind: Element::from_c(kind),
            kids,
        })
    }

    unsafe fn c_str(ch: CHtmlText) -> Result<String> {
        str::from_utf8(
            std::slice::from_raw_parts(ch.text, ch.len as usize)
                .into_iter()
                .map(|i| {
                    if *i < 0 {
                        panic!("Expected unsigned integers, found signed");
                    } else {
                        *i as u8
                    }
                })
                .collect::<Vec<_>>()
                .as_slice(),
        )
        .and_then(|s| Ok(s.to_string()))
        .context("Invalid UTF-8 sequence")
    }

    pub fn to_c(self) -> CHtml {
        match self.kind {
            Element::Text(s) => CHtml {
                node: CHtmlUnion {
                    text: CHtmlText {
                        text: s
                            .as_bytes()
                            .into_iter()
                            .map(|i| *i as i8)
                            .collect::<Vec<i8>>()
                            .as_mut_ptr(),
                        len: s.chars().count() as i32,
                    },
                },
                isNode: 0,
                kids: Vec::new().as_mut_ptr(),
                kid_count: 0,
            },
            _ => {
                let mut chkids: Vec<CHtml> = self
                    .kids
                    .into_iter()
                    .map(|ch| ch.to_c())
                    .filter(|c| c.isNode == 1)
                    .collect();
                let slice = &mut chkids;
                let kids = slice.as_mut_ptr();

                CHtml {
                    isNode: 1,
                    node: CHtmlUnion { kind: self.kind.to_c() },
                    kids,
                    kid_count: chkids.len() as i32,
                }
            }
        }
    }
}

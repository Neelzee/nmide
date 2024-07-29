use core::str;

use c_vec::CVec;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    CElement, CElement_A, CElement_Aside, CElement_Button, CElement_Div, CElement_Input,
    CElement_Nav, CElement_None, CElement_P, CElement_Script, CElement_Section, CElement_Select,
    CElement_Span, CHtml, CHtmlText,
};

#[derive(Debug, TS, Serialize, Deserialize)]
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

#[derive(Debug, TS, Serialize, Deserialize)]
#[ts(export)]
pub struct Html {
    kind: Element,
    kids: Vec<Html>,
}

impl Html {
    pub fn new(kind: Element, kids: Vec<Self>) -> Self {
        Self { kind, kids }
    }

    pub fn from_c(ch: CHtml) -> Self {
        let cvec: CVec<CHtml> = unsafe { CVec::new(ch.kids, ch.kid_count as usize) };

        let mut kids = Vec::new();

        for sch in cvec.iter() {
            kids.push(Self::from_c(*sch));
        }

        Self {
            kind: Element::from_c(ch.kind),
            kids,
        }
    }

    pub fn from_c_text(ch: CHtmlText) -> Self {
        let slice = unsafe { std::slice::from_raw_parts(ch.text, ch.len as usize) };
        let mut vec: Vec<u8> = Vec::new();

        for i in slice {
            if *i < 0i8 {
                panic!("Expected unsigned integers, found signed");
            } else {
                vec.push(*i as u8);
            }
        }

        let buf = vec.as_slice();

        let s = match str::from_utf8(buf) {
            Ok(v) => v,
            Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
        };

        Self {
            kind: Element::Text(s.to_string()),
            kids: Vec::new(),
        }
    }

    pub fn to_c(self) -> CHtml {
        let mut chkids: Vec<CHtml> = self.kids.into_iter().map(|ch| ch.to_c()).collect();
        let slice = &mut chkids;
        let kids = slice.as_mut_ptr();

        CHtml {
            kind: self.kind.to_c(),
            kid_count: chkids.len() as i32,
            kids,
        }
    }
}

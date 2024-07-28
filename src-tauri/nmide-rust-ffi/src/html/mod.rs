use c_vec::CVec;

use crate::{
    CElement, CElement_A, CElement_Aside, CElement_Button, CElement_Div, CElement_Input,
    CElement_Nav, CElement_None, CElement_P, CElement_Script, CElement_Section, CElement_Select,
    CElement_Span, CElement_Text, CHtml,
};

pub enum Element {
    Div,
    P,
    Span,
    Section,
    Input,
    Button,
    Text,
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
            CElement_Text => Self::Text,
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
            Element::Text => CElement_Text,
            Element::Script => CElement_Script,
            Element::Select => CElement_Select,
            Element::Aside => CElement_Aside,
            Element::Nav => CElement_Nav,
            Element::A => CElement_A,
            Element::None => CElement_None,
        }
    }
}

pub struct Html {
    kind: Element,
    kids: Vec<Html>,
}

impl Html {
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

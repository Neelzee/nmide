use std::mem::ManuallyDrop;

use abi_stable::{
    std_types::{ROption, RString, RVec},
    StableAbi,
};

use crate::attr::rattr::RAttr;

macro_rules! rhtmlkind {
    ( $( $name:ident ),* ) => {
        #[repr(u8)]
        #[derive(StableAbi, Clone)]
        pub enum RHtmlKind {
            $(
                $name,
            )*
        }
    };
}

rhtmlkind!(
    Div, P, H1, H2, H3, H4, H5, H6, Span, Section, Article, Aside, Audio, B, Br, Button, Code, Em,
    Fieldset, Form, Img, Input, Label, Link, Li, Menu, Nav, Ol, Option, Select, Style, Svg, Table,
    Td, Th, Ul, Video, Frag, Text, Script
);

#[repr(C)]
#[derive(StableAbi)]
pub struct RHtml {
    pub(crate) kind: RHtmlKind,
    pub(crate) kids: ManuallyDrop<RVec<RHtml>>,
    pub(crate) text: ROption<RString>,
    pub(crate) attrs: ManuallyDrop<RVec<RAttr>>,
}

impl RHtml {
    pub fn new(
        kind: RHtmlKind,
        kids: ManuallyDrop<RVec<RHtml>>,
        text: ROption<RString>,
        attrs: ManuallyDrop<RVec<RAttr>>,
    ) -> Self {
        Self {
            kind,
            kids,
            text,
            attrs,
        }
    }
}

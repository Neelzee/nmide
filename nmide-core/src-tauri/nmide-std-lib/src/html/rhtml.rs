use std::mem::ManuallyDrop;

use abi_stable::{
    std_types::{RString, RVec},
    StableAbi,
};

macro_rules! rhtmlkind {
    ( $( $name:ident ),* ) => {
        #[repr(u8)]
        #[derive(StableAbi)]
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
    Td, Th, Ul, Video, Frag, Text
);

#[repr(C)]
#[derive(StableAbi)]
pub struct RHtml {
    kind: RHtmlKind,
    kids: ManuallyDrop<RVec<RHtml>>,
    text: ManuallyDrop<RString>,
}

impl RHtml {
    pub fn new(
        kind: RHtmlKind,
        kids: ManuallyDrop<RVec<RHtml>>,
        text: ManuallyDrop<RString>,
    ) -> Self {
        Self { kind, kids, text }
    }
}

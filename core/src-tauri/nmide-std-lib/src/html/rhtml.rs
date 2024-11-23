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
                Text,
                Script,
        }

        impl RHtml {
            $(
            #[allow(non_snake_case)]
            pub fn $name(kids: RVec<RHtml>, attrs: RVec<RAttr>) -> Self {
                Self {
                    kind: RHtmlKind::$name,
                    kids,
                    text: ROption::RNone,
                    attrs,
                }
            }
            )*

            pub fn text(text: RString) -> Self {
                Self {
                    kind: RHtmlKind::Text,
                    kids: RVec::new(),
                    text: ROption::RSome(text),
                    attrs: RVec::new(),
                }
            }

            pub fn script(src: RString) -> Self {
                let mut attrs = RVec::new();
                attrs.push(RAttr::new_src(src));
                Self {
                    kind: RHtmlKind::Text,
                    kids: RVec::new(),
                    text: ROption::RNone,
                    attrs,
                }
            }
        }
    };
}

rhtmlkind!(
    Div, P, H1, H2, H3, H4, H5, H6, Span, Section, Article, Aside, Audio, B, Br, Button, Code, Em,
    Fieldset, Form, Img, Input, Label, Link, Li, Menu, Nav, Ol, Option, Select, Style, Svg, Table,
    Td, Tr, Th, Ul, Video, Frag, Tbody
);

#[repr(C)]
#[derive(StableAbi)]
pub struct RHtml {
    pub(crate) kind: RHtmlKind,
    pub(crate) kids: RVec<RHtml>,
    pub(crate) text: ROption<RString>,
    pub(crate) attrs: RVec<RAttr>,
}

impl RHtml {
    pub fn new(
        kind: RHtmlKind,
        kids: RVec<RHtml>,
        text: ROption<RString>,
        attrs: RVec<RAttr>,
    ) -> Self {
        Self {
            kind,
            kids,
            text,
            attrs,
        }
    }
}

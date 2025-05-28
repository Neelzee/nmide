//! RHtml

// TODO: Add doc-string

use std::str::FromStr;

use crate::attr::rs_attrs::RAttr;
use abi_stable::{
    StableAbi,
    std_types::{ROption, RString, RVec},
};
use core_std_lib::html::Html;

macro_rules! rhtmlkind {
    ( $( $name:ident ),* ) => {
        #[repr(u8)]
        #[derive(StableAbi, Clone, Copy)]
        pub enum RHtmlKind {
            $(
                $name,
            )*
        }

        impl RHtmlKind {
            pub fn from_html(html: &Html) -> Self {
                match html {
                    $(
                    Html::$name { .. } => Self::$name,
                    )*
                }
            }
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

            pub fn to_html(self) -> Html {
                match &self.kind {
                    $(
                        RHtmlKind::$name => {
                            let mut html = Html::$name();
                            let kids = self.kids.into_iter().map(|k| k.to_html()).collect();
                            html = html.replace_kids(kids);
                            for a in self.attrs {
                                let attr = a.to_attr();
                                html = html.add_attr(attr);
                            }
                            if let ROption::RSome(txt) = self.text {
                                html = html.set_text(txt.as_str().to_string());
                            }
                            html
                        }
                    )*
                }
            }
        }
    };
}

rhtmlkind!(
    Div, P, H1, H2, H3, H4, H5, H6, Span, Section, Article, Aside, Audio, B, Br, Button, Code, Em,
    Fieldset, Form, Img, Input, Label, Link, Li, Menu, Nav, Ol, Option, Select, Style, Svg, Table,
    Td, Tr, Th, Ul, Video, Frag, Tbody, Main, Script, TextArea, Thead, Strong
);

#[repr(C)]
#[derive(StableAbi, Clone)]
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

    pub fn empty() -> Self {
        Self {
            kind: RHtmlKind::Div,
            kids: RVec::new(),
            text: ROption::RNone,
            attrs: RVec::new(),
        }
    }
}

impl From<Html> for RHtml {
    fn from(value: Html) -> Self {
        Self {
            kind: RHtmlKind::from_html(&value),
            kids: value.kids().into_iter().map(|v| v.into()).collect(),
            text: if value.text().len() == 0 {
                ROption::RNone
            } else {
                ROption::RSome(RString::from_str(&value.text()).unwrap_or_default())
            },
            attrs: value.attrs().into_iter().map(|v| v.into()).collect(),
        }
    }
}

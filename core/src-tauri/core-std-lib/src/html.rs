//! *Html
//!
//! An Html type is an representation of an HTML-Element. This mapping is close to total.

use crate::attrs::Attr;

macro_rules! htmlkind {
        ( $( $name:ident ),* ) => {
            pub enum THtmlKind {
                $(
                    $name,
                )*
            }
        };
    }

htmlkind!(
    Div, P, H1, H2, H3, H4, H5, H6, Span, Section, Article, Aside, Audio, B, Br, Button, Code, Em,
    Fieldset, Form, Img, Input, Label, Link, Li, Menu, Nav, Ol, Option, Select, Style, Svg, Table,
    Td, Th, Tr, Ul, Video, Frag, Text, Script, Tbody, Main
);

pub struct Html {
    kind: THtmlKind,
    kids: Vec<Html>,
    text: Option<String>,
    attrs: Vec<Attr>,
}

impl Html {
    pub fn empty() -> Self {
        todo!()
    }
}

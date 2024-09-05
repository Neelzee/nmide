use std::{mem::ManuallyDrop, ptr::null};

use crate::html::Html;

macro_rules! chtmlkind {
    ( $( $name:ident ),* ) => {
        #[stabby::stabby]
        #[repr(u8)]
        pub enum CHtmlKind {
            $(
                $name,
            )*
            Text,
        }

        impl CHtmlKind {
            pub fn from_html(html: &Html) -> CHtmlKind {
                match html {
                    $(
                        Html::$name { .. } => CHtmlKind::$name,
                    )*
                    Html::Text(_) => CHtmlKind::Text,
                }
            }
        }
    };
}

chtmlkind!(
    Div, P, H1, H2, H3, H4, H5, H6, Span, Section, Article, Aside, Audio, B, Br, Button, Code, Em,
    Fieldset, Form, Img, Input, Label, Link, Li, Menu, Nav, Ol, Option, Select, Style, Svg, Table,
    Td, Th, Ul, Video, Frag
);

#[stabby::stabby]
pub struct CHtml {
    kind: CHtmlKind,
    kids: *const ManuallyDrop<CHtml>,
    text: *const u8,
}

impl CHtml {
    pub fn new(
        kind: CHtmlKind,
        kids: ManuallyDrop<Vec<ManuallyDrop<CHtml>>>,
        text: ManuallyDrop<String>,
    ) -> Self {
        Self {
            kind,
            kids: kids.as_ptr(),
            text: text.as_bytes().as_ptr(),
        }
    }
}

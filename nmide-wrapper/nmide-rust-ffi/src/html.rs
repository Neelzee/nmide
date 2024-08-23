use std::{borrow::Borrow, collections::VecDeque, ffi::CString, ptr::null_mut};

use crate::{
    util::from_char, CHtml, CHtmlContent, CHtmlElement, CHtmlTag_Article, CHtmlTag_Aside,
    CHtmlTag_Audio, CHtmlTag_B, CHtmlTag_Br, CHtmlTag_Button, CHtmlTag_Code, CHtmlTag_Div,
    CHtmlTag_Em, CHtmlTag_Fieldset, CHtmlTag_Form, CHtmlTag_Frag, CHtmlTag_H1, CHtmlTag_H2,
    CHtmlTag_H3, CHtmlTag_H4, CHtmlTag_H5, CHtmlTag_H6, CHtmlTag_Img, CHtmlTag_Input,
    CHtmlTag_Label, CHtmlTag_Li, CHtmlTag_Link, CHtmlTag_Menu, CHtmlTag_Nav, CHtmlTag_None,
    CHtmlTag_Ol, CHtmlTag_Option, CHtmlTag_P, CHtmlTag_Section, CHtmlTag_Select, CHtmlTag_Span,
    CHtmlTag_Style, CHtmlTag_Svg, CHtmlTag_Table, CHtmlTag_Td, CHtmlTag_Th, CHtmlTag_Ul,
    CHtmlTag_Video,
};
use anyhow::Result;
use nmide_std_lib::html::Html;

impl CHtml {
    pub fn html_from(value: &Html) -> Result<Self> {
        let (content, isElement) = match value {
            Html::Text(s) => {
                let cstring = CString::new(s.to_string())?;
                let cstr = cstring.as_c_str();
                (
                    CHtmlContent {
                        text: cstr.as_ptr(),
                    },
                    false,
                )
            }
            _ => (
                CHtmlContent {
                    element: CHtmlElement {
                        tag: to_tag(&value),
                        children: value
                            .kids()
                            .iter()
                            .filter_map(|k| CHtml::html_from(k).ok())
                            .collect::<Vec<_>>()
                            .as_mut_ptr(),
                        len: value.kids().len(),
                    },
                },
                true,
            ),
        };
        Ok(CHtml { content, isElement })
    }

    pub fn to_html(&self) -> Result<Html> {
        if self.isElement {
            Ok(Html::Frag {
                kids: unsafe {
                    Vec::from_raw_parts(
                        self.content.element.children,
                        self.content.element.len,
                        std::mem::size_of::<CHtml>(),
                    )
                }
                .iter()
                .filter_map(|k| k.to_html().ok())
                .collect(),
                attrs: Vec::new(),
            }
            .cast_html(from_tag(unsafe { self.content.element }.tag)))
        } else {
            Ok(Html::Text(unsafe { from_char(self.content.text) }?))
        }
    }
}

pub fn from_tag(tag: u32) -> Html {
    match tag {
        CHtmlTag_Div => Html::Div {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_P => Html::P {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_H1 => Html::H1 {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_H2 => Html::H2 {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_H3 => Html::H3 {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_H4 => Html::H4 {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_H5 => Html::H5 {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_H6 => Html::H6 {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Span => Html::Span {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Section => Html::Section {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Article => Html::Article {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Aside => Html::Aside {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Audio => Html::Audio {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_B => Html::B {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Br => Html::Br {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Button => Html::Button {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Code => Html::Code {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Em => Html::Em {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Fieldset => Html::Fieldset {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Form => Html::Form {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Img => Html::Img {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Input => Html::Input {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Label => Html::Label {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Link => Html::Link {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Li => Html::Li {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Menu => Html::Menu {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Nav => Html::Nav {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Ol => Html::Ol {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Option => Html::Option {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Select => Html::Select {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Style => Html::Style {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Svg => Html::Svg {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Table => Html::Table {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Td => Html::Td {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Th => Html::Th {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Ul => Html::Ul {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Video => Html::Video {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_Frag => Html::Frag {
            kids: Vec::new(),
            attrs: Vec::new(),
        },
        CHtmlTag_None => Html::Text(String::new()),
        _ => Html::Text(String::new()), // Default case for unknown tags
    }
}

pub fn to_tag(element: &Html) -> u32 {
    match element {
        Html::Div { kids: _, attrs: _ } => CHtmlTag_Div,
        Html::P { kids: _, attrs: _ } => CHtmlTag_P,
        Html::H1 { kids: _, attrs: _ } => CHtmlTag_H1,
        Html::H2 { kids: _, attrs: _ } => CHtmlTag_H2,
        Html::H3 { kids: _, attrs: _ } => CHtmlTag_H3,
        Html::H4 { kids: _, attrs: _ } => CHtmlTag_H4,
        Html::H5 { kids: _, attrs: _ } => CHtmlTag_H5,
        Html::H6 { kids: _, attrs: _ } => CHtmlTag_H6,
        Html::Span { kids: _, attrs: _ } => CHtmlTag_Span,
        Html::Section { kids: _, attrs: _ } => CHtmlTag_Section,
        Html::Article { kids: _, attrs: _ } => CHtmlTag_Article,
        Html::Aside { kids: _, attrs: _ } => CHtmlTag_Aside,
        Html::Audio { kids: _, attrs: _ } => CHtmlTag_Audio,
        Html::B { kids: _, attrs: _ } => CHtmlTag_B,
        Html::Br { kids: _, attrs: _ } => CHtmlTag_Br,
        Html::Button { kids: _, attrs: _ } => CHtmlTag_Button,
        Html::Code { kids: _, attrs: _ } => CHtmlTag_Code,
        Html::Em { kids: _, attrs: _ } => CHtmlTag_Em,
        Html::Fieldset { kids: _, attrs: _ } => CHtmlTag_Fieldset,
        Html::Form { kids: _, attrs: _ } => CHtmlTag_Form,
        Html::Img { kids: _, attrs: _ } => CHtmlTag_Img,
        Html::Input { kids: _, attrs: _ } => CHtmlTag_Input,
        Html::Label { kids: _, attrs: _ } => CHtmlTag_Label,
        Html::Link { kids: _, attrs: _ } => CHtmlTag_Link,
        Html::Li { kids: _, attrs: _ } => CHtmlTag_Li,
        Html::Menu { kids: _, attrs: _ } => CHtmlTag_Menu,
        Html::Nav { kids: _, attrs: _ } => CHtmlTag_Nav,
        Html::Ol { kids: _, attrs: _ } => CHtmlTag_Ol,
        Html::Option { kids: _, attrs: _ } => CHtmlTag_Option,
        Html::Select { kids: _, attrs: _ } => CHtmlTag_Select,
        Html::Style { kids: _, attrs: _ } => CHtmlTag_Style,
        Html::Svg { kids: _, attrs: _ } => CHtmlTag_Svg,
        Html::Table { kids: _, attrs: _ } => CHtmlTag_Table,
        Html::Td { kids: _, attrs: _ } => CHtmlTag_Td,
        Html::Th { kids: _, attrs: _ } => CHtmlTag_Th,
        Html::Ul { kids: _, attrs: _ } => CHtmlTag_Ul,
        Html::Video { kids: _, attrs: _ } => CHtmlTag_Video,
        Html::Frag { kids: _, attrs: _ } => CHtmlTag_Frag,
        Html::Text(_) => CHtmlTag_None,
    }
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

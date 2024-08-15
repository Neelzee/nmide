use std::{borrow::Borrow, collections::VecDeque, ffi::CString, ptr::null_mut};

use crate::{
    attr::Attr, util::from_char, CHtml, CHtmlContent, CHtmlElement, CHtmlTag_Div, CHtmlTag_None,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub enum Html {
    Div { kids: Vec<Html>, attrs: Vec<Attr> },
    Btn { kids: Vec<Html>, attrs: Vec<Attr> },
    Frag { kids: Vec<Html>, attrs: Vec<Attr> },
    Text(String),
    None,
}

fn lookup<K, V>(xs: &[(K, V)], x: K) -> Option<&V>
where
    K: Eq,
{
    match xs {
        [] => None,
        [(y, v), ..] if y == &x => Some(v),
        [_, ys @ ..] => lookup(ys, x),
    }
}

impl Html {
    pub fn apply_if<F, G>(&mut self, f: F, g: G)
    where
        F: Fn(&Html) -> bool,
        G: FnOnce(&mut Html),
    {
        let mut queue = VecDeque::new();
        queue.push_back(self);
        while let Some(node) = queue.pop_front() {
            if f(node) {
                g(node);
                break;
            }
            match node {
                Html::Div { kids, .. } => {
                    for k in kids {
                        queue.push_back(k);
                    }
                }
                Html::Btn { kids, .. } => {
                    for k in kids {
                        queue.push_back(k);
                    }
                }
                Html::Frag { kids, .. } => {
                    for k in kids {
                        queue.push_back(k);
                    }
                }
                Html::Text(_) => (),
                Html::None => (),
            }
        }
    }

    pub fn get_all<F>(self, f: F) -> Vec<Html>
    where
        F: Fn(&Html) -> bool,
    {
        let mut queue = VecDeque::new();
        queue.push_back(&self);
        let mut result: Vec<Html> = Vec::new();
        while let Some(node) = queue.pop_front() {
            if f(node) {
                result.push(node.clone());
            }
            match node {
                Html::Div { kids, .. } => {
                    for k in kids {
                        queue.push_back(k);
                    }
                }
                Html::Btn { kids, .. } => {
                    for k in kids {
                        queue.push_back(k);
                    }
                }
                Html::Frag { kids, .. } => {
                    for k in kids {
                        queue.push_back(k);
                    }
                }
                Html::Text(_) => (),
                Html::None => (),
            }
        }
        return result;
    }

    pub fn adopt(&mut self, kid: Html) {
        match self {
            Html::Div { kids, .. } => kids.push(kid),
            Html::Btn { kids, .. } => kids.push(kid),
            Html::Frag { kids, .. } => kids.push(kid),
            Html::Text(_) => (),
            Html::None => (),
        }
    }

    pub fn get_attr(&self, attr: &str) -> Option<Attr> {
        let attrs = match self {
            Html::Div { attrs, .. } => Some(attrs),
            Html::Btn { attrs, .. } => Some(attrs),
            Html::Frag { attrs, .. } => Some(attrs),
            Html::Text(_) => None,
            Html::None => None,
        };

        attrs.and_then(|attrs| {
            match lookup(
                attrs
                    .into_iter()
                    .map(|a| (a.to_string_id(), a))
                    .collect::<Vec<_>>()
                    .as_slice(),
                attr,
            ) {
                Some(a) => Some(<Attr>::clone(a)),
                None => None,
            }
        })
    }

    /// .
    ///
    /// # Errors
    ///
    /// This function will return an error if .
    ///
    /// # Safety
    ///
    /// .
    pub unsafe fn from_c(ch: CHtml) -> Result<Self> {
        let content = ch.content;

        if !ch.isElement {
            return Ok(Self::Text(from_char(
                content.text, // unsafe
            )?));
        }
        let element = content.element;
        match element.tag {
            CHtmlTag_Div => Ok(Self::Div {
                kids: Vec::from_raw_parts(
                    // unsafe
                    element.children,
                    element.len,
                    std::mem::size_of::<CHtml>(),
                )
                .into_iter()
                .filter_map(|ptr| {
                    Self::from_c(ptr).ok() // unsafe
                })
                .collect(),

                attrs: Vec::new(),
            }),
            _ => Ok(Self::None),
        }
    }

    pub fn to_c(self) -> Result<CHtml> {
        match self {
            Html::Div { kids, .. } => {
                let len = kids.len();
                Ok(CHtml {
                    content: CHtmlContent {
                        element: CHtmlElement {
                            tag: CHtmlTag_Div,
                            children: kids
                                .into_iter()
                                .filter_map(|k| Self::to_c(k).ok())
                                .collect::<Vec<_>>()
                                .as_mut_ptr(),
                            len,
                        },
                    },
                    isElement: true,
                })
            }
            Html::Text(s) => Ok(CHtml {
                content: CHtmlContent {
                    text: CString::new(s.clone())?.as_c_str().as_ptr(),
                },
                isElement: false,
            }),
            _ => Ok(CHtml {
                content: CHtmlContent {
                    element: CHtmlElement {
                        tag: CHtmlTag_None,
                        children: null_mut(),
                        len: 0,
                    },
                },
                isElement: true,
            }),
        }
    }

    pub fn get_text(&self) -> String {
        match self {
            Html::Div { kids, .. } => kids.iter().map(Self::get_text).collect::<String>(),
            Html::Btn { kids, .. } => kids.iter().map(Self::get_text).collect::<String>(),
            Html::Frag { kids, .. } => kids.iter().map(Self::get_text).collect::<String>(),
            Html::Text(s) => s.clone(),
            Html::None => String::new(),
        }
    }
}

#[test]
fn to_and_from_html() -> Result<()> {
    let html = Html::Text("Hello, World!".to_string());

    let chtml = html.to_c()?;

    let res = unsafe { Html::from_c(chtml) };
    assert!(res.is_ok());
    assert_eq!(res.unwrap().get_text(), "Hello, World!");

    Ok(())
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

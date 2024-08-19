use std::collections::VecDeque;

use crate::{attr::Attr, utils::fst};

#[derive(Debug, Clone)]
pub enum Element {
    Div,
    P,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Span,
    Section,
    Article,
    Aside,
    Comment,
    Audio,
    B,
    Br,
    Button,
    Code,
    Em,
    Fieldset,
    Form,
    Img,
    Input,
    Label,
    Link,
    Li,
    Menu,
    Nav,
    Ol,
    Option,
    Select,
    Style,
    Svg,
    Table,
    Td,
    Th,
    Ul,
    Video,
}

#[derive(Debug, Clone)]
pub struct Html {
    kind: Element,
    kids: Vec<Html>,
    attrs: Vec<Attr>,
}

impl Html {
    pub fn insert_id(self, other: Html, id: &str) -> Self {
        self.apply_if(
            |k| k.attrs.iter().any(|a| a.to_id().is_some_and(|i| i == id)),
            |h| h.adopt(other)
        )
    }

    /// Applies the given function G to the Tree, if the given
    /// predicate F is true.
    pub fn apply_if<F, G>(self, f: F, g: G) -> Self
    where
        F: Fn(&Self) -> bool + Clone,
        G: FnOnce(Self) -> Self + Clone,
    {
        fst(self._apply_if(f, g, false))
    }

    /// Helper function
    fn _apply_if<F, G>(self, f: F, g: G, applied: bool) -> (Self, bool)
    where
        F: Fn(&Self) -> bool + Clone,
        G: FnOnce(Self) -> Self + Clone,
    {
        if f(&self) {
            return (g(self), true);
        }

        let (kids, a) =
            self.kids
                .into_iter()
                .fold((Vec::new(), applied), |(mut kids, a), kid| {
                let (k, new_a) = kid._apply_if(f.clone(), g.clone(), a);
                kids.push(k);
                (kids, new_a)
                });
        
        (Self { kids, ..self }, a)
    }

    pub fn adopt(self, other: Self) -> Self {
        Self {
            kids: {
                let mut kids = self.kids;
                kids.push(other);
                kids
            },
            ..self
        }
    }

    pub fn kids_dfs(&self) -> Vec<&Html> {
        match self.kids.as_slice() {
            [] => Vec::new(),
            [x, xs @ ..] => {
                let mut vec = vec![x];
                let mut kids: Vec<&Html> = x.kids.iter().flat_map(|k| k.kids_dfs()).collect();
                vec.append(&mut kids);
                let mut res = xs.iter().flat_map(|k| k.kids_dfs()).collect::<Vec<_>>();
                vec.append(&mut res);
                vec
            }
        }
    }
}

// TODO: I prefer enums over structs
enum Foo {
    Html { kids: Vec<Foo>, attr: Vec<Attr> },
    Frag { kids: Vec<Foo>, attr: Vec<Attr> },
    Text(String),
}

impl Foo {
pub fn apply_if<F, G>(self, f: F, g: G) -> Self
    where
        F: Fn(&Self) -> bool + Clone,
        G: FnOnce(Self) -> Self + Clone,
    {
        fst(self._apply_if(f, g, false))
    }

    /// Helper function
    fn _apply_if<F, G>(self, f: F, g: G, applied: bool) -> (Self, bool)
    where
        F: Fn(&Self) -> bool + Clone,
        G: FnOnce(Self) -> Self + Clone,
    {
        match (self, applied) {
            res @ (_, true) => res,
            (node, _) if f(&node) => (g(node), true),
            res @ (Self::Text(_), _) => res,
            (node, _) => {
                let (kids, a) = node.kids()
                    .into_iter()
                    .fold((Vec::new(), applied), |(mut kids, a), kid| {
                        let (k, new_a) = kid._apply_if(f.clone(), g.clone(), a);
                        kids.push(k);
                        (kids, new_a)
                        });
                (node.replace_kids(kids), a)
            }
        }
    }

    fn replace_kids(self, kids: Vec<Self>) -> Self {
        todo!()
    }

    

    fn kids(&self) -> Vec<Self> {
        todo!()
    }
}

impl Into<HtmlBFSIter> for Html {
    fn into(self) -> HtmlBFSIter {
        HtmlIter {
            nxt: Some(self),
            rem: VecDeque::new(),
        }
        .into()
    }
}

pub(crate) struct HtmlIter {
    nxt: Option<Html>,
    rem: VecDeque<Html>,
}

pub(crate) struct HtmlBFSIter(HtmlIter);

impl From<HtmlIter> for HtmlBFSIter {
    fn from(value: HtmlIter) -> Self {
        Self(value)
    }
}

impl Iterator for HtmlBFSIter {
    type Item = Html;

    fn next(&mut self) -> Option<Self::Item> {
        let pre = self.0.nxt.clone();
        match &pre {
            Some(sk) => {
                for k in &sk.kids {
                    self.0.rem.push_back(k.clone());
                }
            }
            None => (),
        }
        self.0.nxt = self.0.rem.pop_front();
        pre
    }
}

pub(crate) struct HtmlDFSIter(HtmlIter);

impl From<HtmlIter> for HtmlDFSIter {
    fn from(value: HtmlIter) -> Self {
        Self(value)
    }
}

impl Iterator for HtmlDFSIter {
    type Item = Html;

    fn next(&mut self) -> Option<Self::Item> {
        let pre = self.0.nxt.clone();
        match &pre {
            Some(sk) => {
                for k in sk.kids_dfs() {
                    self.0.rem.push_back(k.clone());
                }
            }
            None => (),
        }
        self.0.nxt = self.0.rem.pop_front();
        pre
    }
}

use std::collections::VecDeque;

use crate::{
    attr::Attr,
    css::{Style, Unit},
    html::Html::Text,
    utils::{fst, snd},
};
use nmide_macros::{css, define_html};

pub fn foo() {
    let pad: Vec<_> = css!(Padding 32; Ø, Width 1.0; Px);
    println!("{pad:?}");
}

define_html!(
    Div, P, H1, H2, H3, H4, H5, H6, Span, Section, Article, Aside, Audio, B, Br, Button, Code, Em,
    Fieldset, Form, Img, Input, Label, Link, Li, Menu, Nav, Ol, Option, Select, Style, Svg, Table,
    Td, Th, Ul, Video
);

impl Html {
    pub fn insert_id(self, other: Html, id: &str) -> Self {
        self.apply_if(
            |k| k.attrs().iter().any(|a| a.to_id().is_some_and(|i| i == id)),
            |h| h.adopt(other),
        )
    }

    /// Applies the given function G, if the given predicate F evaluates to true
    /// on the tree. G is only applied once.
    ///
    /// # Example
    /// ```rust
    /// let html = Html::Div {
    ///         kids: vec![
    ///                 Html::P(),
    ///                 Html::P {
    ///                     kids: Vec::new(),
    ///                     attrs: attrs!(Id "foobar")
    ///                 },
    ///                 Html::P(),
    ///             ],
    ///         attrs: Vec::new()
    ///     };
    ///
    /// println!(
    ///     "{:?}",
    ///     html.apply_if(
    ///         |n| n.attrs().contains(attr!(Id "foobar")),
    ///         |h| h.adopt(Html::Text("Hello, World!"))
    ///     );
    /// /*
    ///     Html::Div {
    ///         kids: vec![
    ///                 Html::P(),
    ///                 Html::P {
    ///                     kids: vec![Html::Text("Hello, World!")],
    ///                     attrs: attrs!(Id "foobar")
    ///                 },
    ///                 Html::P(),
    ///             ],
    ///         attrs: Vec::new()
    ///     };
    /// */
    /// ```
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
                let (kids, a) =
                    node.kids()
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

    pub fn map<F>(self, f: F) -> Self
    where
        F: Fn(Self) -> Self + Clone,
    {
        f(self.clone()).replace_kids(
            f(self)
                .kids()
                .into_iter()
                .map(|k| k.map(f.clone()))
                .collect(),
        )
    }

    /// Returns true if any node in the Html-tree fulfils the given predicate
    pub fn any<P>(self, p: P) -> bool
    where
        P: Fn(&Self) -> bool + Clone,
    {
        snd(self._any(p, false))
    }

    /// Helper function
    fn _any<P>(self, p: P, found: bool) -> (Self, bool)
    where
        P: Fn(&Self) -> bool + Clone,
    {
        match (self, found) {
            res @ (_, true) => res,
            (node, _) if p(&node) => (node, true),
            res @ (Self::Text(_), _) => res,
            (node, _) => (
                node.clone(),
                node.kids()
                    .into_iter()
                    .fold(found, |a, kid| snd(kid._any(p.clone(), a))),
            ),
        }
    }

    pub fn kids_dfs(&self) -> Vec<Html> {
        match self.kids().as_slice() {
            [] => Vec::new(),
            [x, xs @ ..] => {
                let mut vec = vec![x.clone()];
                let mut kids: Vec<Html> = x.kids().iter().flat_map(|k| k.kids_dfs()).collect();
                vec.append(&mut kids);
                let mut res = xs.iter().flat_map(|k| k.kids_dfs()).collect::<Vec<_>>();
                vec.append(&mut res);
                vec
            }
        }
    }

    pub fn into_bfs_iter(self) -> HtmlBFSIter {
        self.into()
    }

    pub fn into_dfs_iter(self) -> HtmlDFSIter {
        self.into()
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

pub struct HtmlIter {
    nxt: Option<Html>,
    rem: VecDeque<Html>,
}

pub struct HtmlBFSIter(HtmlIter);

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
                for k in sk.kids() {
                    self.0.rem.push_back(k);
                }
            }
            None => (),
        }
        self.0.nxt = self.0.rem.pop_front();
        pre
    }
}

impl Into<HtmlDFSIter> for Html {
    fn into(self) -> HtmlDFSIter {
        HtmlIter {
            nxt: Some(self),
            rem: VecDeque::new(),
        }
        .into()
    }
}

pub struct HtmlDFSIter(HtmlIter);

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

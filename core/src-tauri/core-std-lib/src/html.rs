//! *Html
//!
//! An Html type is an representation of an HTML-Element.

use crate::attrs::Attr;
use core_macros::define_html;
use serde::{Deserialize, Serialize};
use crate::instruction::Instruction;
use ts_rs::TS;

define_html!(
    attr_type = Attr,
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
    #[serde(rename_all = "camelCase")]
    #[ts(export)]
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
    Tr,
    Ul,
    Video,
    Frag,
    Script,
    Tbody,
    Main
);

impl Html {
    pub fn add_attr(self, attr: Attr) -> Self {
        let mut attrs = self.attrs();
        attrs.push(attr);
        self.set_attrs(attrs)
    }

    pub fn has_attr(&self, attr: &str) -> bool {
        self.attrs().iter().any(|a| a.is(attr))
    }

    pub fn set_attr(self, attr: &str, new_attr: Attr) -> Self {
        let mut attrs: Vec<Attr> = self.attrs().into_iter().filter(|a| !a.is(attr)).collect();
        attrs.push(new_attr);
        self.set_attrs(attrs)
    }

    pub fn rem_attr(self, attr: &str) -> Self {
        let attrs: Vec<Attr> = self.attrs().into_iter().filter(|a| !a.is(attr)).collect();
        self.set_attrs(attrs)
    }

    pub(crate) fn modify<F, P>(self, f: F, p: P) -> Self
    where
        F: Fn(Self) -> Self + Clone,
        P: Fn(&Self) -> bool + Clone,
    {
        if p(&self) {
            f(self)
        } else {
            let new_kids = self.kids().into_iter().map(|k| k.modify(f.clone(), p.clone())).collect();
            self.replace_kids(new_kids)
        }
    }

    pub fn cmp_id(&self, ui: &str) -> bool {
        match self.attrs().into_iter().find(|a| a.is("id")) {
            Some(Attr::Id(s)) => s == ui,
            _ => false,
        }
    }

    pub fn cmp_class(&self, class: &str) -> bool {
        match self.attrs().into_iter().find(|a| a.is("class")) {
            Some(Attr::Class(s)) => s == class,
            _ => false,
        }
    }
}

pub struct UIInstructionBuilder {
    count: usize,
    node: Vec<(usize, Instruction<Html>)>,
    text: Vec<(usize, Instruction<String>)>,
    attr: Vec<(usize, Instruction<Attr>)>,
}

impl Default for UIInstructionBuilder {
    fn default() -> Self {
        Self {
            count: 0,
            node: Vec::new(),
            text: Vec::new(),
            attr: Vec::new(),
        }
    }
}

impl UIInstructionBuilder {
    pub(crate) fn new(inst: (Vec<(usize, Instruction<Html>)>, Vec<(usize, Instruction<String>)>, Vec<(usize, Instruction<Attr>)>)) -> Self {
        let (node, text, attr) = inst;
        Self {
            count: 0,
            node,
            text,
            attr
        }
    }

    pub fn instruction(&self) -> (Vec<(usize, Instruction<Html>)>, Vec<(usize, Instruction<String>)>, Vec<(usize, Instruction<Attr>)>) {
        (self.node.clone(), self.text.clone(), self.attr.clone())
    }

    pub fn set_text(self, id: Option<String>, class: Option<String>, text: String) -> Self {
        let mut texts = self.text;
        texts.push((self.count, Instruction::Add(id, class, text)));
        Self {
            count: self.count + 1,
            text: texts,
            ..self
        }
    }

    pub fn add_node(self, ui: Html, id: Option<String>, class: Option<String>) -> Self {
        let mut nodes = self.node;
        nodes.push((self.count, Instruction::Add(id, class, ui)));
        Self {
            count: self.count + 1,
            node: nodes,
            ..self
        }
    }

    pub fn rem_node(self, id: Option<String>, class: Option<String>) -> Self {
        let mut nodes = self.node;
        nodes.push((self.count, Instruction::Rem(id, class, Html::Div())));
        Self {
            count: self.count + 1,
            node: nodes,
            ..self
        }
    }

    pub fn add_attr(
        self,
        id: Option<String>,
        class: Option<String>,
        attr: Attr,
    ) -> Self {
        let mut attrs = self.attr;
        attrs.push((self.count, Instruction::Add(id, class, attr)));
        Self {
            count: self.count + 1,
            attr: attrs,
            ..self
        }
    }

    pub fn rem_attr(self, attr: Attr, id: Option<String>, class: Option<String>) -> Self {
        let mut attrs = self.attr;
        attrs.push((self.count, Instruction::Rem(id, class, attr)));
        Self {
            count: self.count + 1,
            attr: attrs,
            ..self
        }
    }

    pub fn set_attr(
        self,
        id: Option<String>,
        class: Option<String>,
        attr: Attr,
    ) -> Self {
        let mut attrs = self.attr;
        attrs.push((self.count, Instruction::Mod(id, class, attr)));
        Self {
            count: self.count + 1,
            attr: attrs,
            ..self
        }
    }
    fn node_instruction(node: Html, inst: Instruction<Html>) -> Html {
        match inst {
            Instruction::NoOp => node,
            // ADD
            Instruction::Add(Some(id), Some(class), child) => node.modify(
                |n| { n.adopt(child.clone()) },
                |n| { n.cmp_id(&id) && n.cmp_class(&class) }
            ),
            Instruction::Add(Some(id), _, child) => node.modify(
                |n| { n.adopt(child.clone()) },
                |n| { n.cmp_id(&id) }
            ),
            Instruction::Add(_, Some(class), child) => node.modify(
                |n| { n.adopt(child.clone()) },
                |n| { n.cmp_class(&class) }
            ),
            Instruction::Add(_, _, child) => node,
            // REM
            Instruction::Rem(oid, ocl, _) => {
                let child = |n: &Html| {
                    match (oid.clone(), ocl.clone()) {
                        (Some(id), Some(class)) =>
                            n.cmp_id(&id) && n.cmp_class(&class),
                        (Some(id), None) =>
                            n.cmp_id(&id),
                        (None, Some(class)) =>
                            n.cmp_class(&class),
                        _ => false,
                    }
                };
                let parent = |p: &Html| p.kids().iter().any(|k| child(k));
                node.modify(
                    |p| {
                        let kids = p.kids().into_iter().filter(child).collect();
                        p.replace_kids(kids)
                    },
                    parent
                )
            }
            Instruction::Mod(_, _, _) => node,
            Instruction::Then(f, s) =>
                Self::node_instruction(Self::node_instruction(node, *f), *s)
        }
    }

    fn text_instruction(node: Html, inst: Instruction<String>) -> Html {
        match inst {
            Instruction::NoOp => node,
            // ADD
            Instruction::Add(Some(id), Some(class), text) => node.modify(
                |n| { n.set_text(text.clone()) },
                |n| { n.cmp_id(&id) && n.cmp_class(&class) }
            ),
            Instruction::Add(Some(id), _, text) => node.modify(
                |n| { n.set_text(text.clone()) },
                |n| { n.cmp_id(&id) }
            ),
            Instruction::Add(_, Some(class), text) => node.modify(
                |n| { n.set_text(text.clone()) },
                |n| { n.cmp_class(&class) }
            ),
            Instruction::Add(_, _, child) => node,
            // REM
            Instruction::Rem(oid, ocl, _) => {
                let p = |n: &Html| {
                    match (oid.clone(), ocl.clone()) {
                        (Some(id), Some(class)) =>
                            n.cmp_id(&id) && n.cmp_class(&class),
                        (Some(id), None) =>
                            n.cmp_id(&id),
                        (None, Some(class)) =>
                            n.cmp_class(&class),
                        _ => false,
                    }
                };
                node.modify(
                    |n| n.set_text(""),
                    p
                )
            }
            Instruction::Mod(oid, ocl, txt) => {
                let p = |n: &Html| {
                    match (oid.clone(), ocl.clone()) {
                        (Some(id), Some(class)) =>
                            n.cmp_id(&id) && n.cmp_class(&class),
                        (Some(id), None) =>
                            n.cmp_id(&id),
                        (None, Some(class)) =>
                            n.cmp_class(&class),
                        _ => false,
                    }
                };
                node.modify(
                    |n| {
                        let t = n.text();
                        n.set_text(format!("{}{}", t, txt))
                    },
                    p
                )
            }
            Instruction::Then(f, s) =>
                Self::text_instruction(Self::text_instruction(node, *f), *s)
        }
    }

    fn attr_instruction(node: Html, inst: Instruction<Attr>) -> Html {
        match inst {
            Instruction::NoOp => node,
            // ADD
            Instruction::Add(Some(id), Some(class), attr) => node.modify(
                |n| { n.add_attr(attr.clone()) },
                |n| { n.cmp_id(&id) && n.cmp_class(&class) }
            ),
            Instruction::Add(Some(id), _, attr) => node.modify(
                |n| { n.add_attr(attr.clone()) },
                |n| { n.cmp_id(&id) }
            ),
            Instruction::Add(_, Some(class), attr) => node.modify(
                |n| { n.add_attr(attr.clone()) },
                |n| { n.cmp_class(&class) }
            ),
            Instruction::Add(_, _, child) => node,
            // REM
            Instruction::Rem(oid, ocl, attr) => {
                let p = |n: &Html| {
                    match (oid.clone(), ocl.clone()) {
                        (Some(id), Some(class)) =>
                            n.cmp_id(&id) && n.cmp_class(&class),
                        (Some(id), None) =>
                            n.cmp_id(&id),
                        (None, Some(class)) =>
                            n.cmp_class(&class),
                        _ => false,
                    }
                };
                node.modify(
                    |n| n.rem_attr(attr.as_string_rep()),
                    p
                )
            }
            Instruction::Mod(oid, ocl, attr) => {
                let p = |n: &Html| {
                    match (oid.clone(), ocl.clone()) {
                        (Some(id), Some(class)) =>
                            n.cmp_id(&id) && n.cmp_class(&class),
                        (Some(id), None) =>
                            n.cmp_id(&id),
                        (None, Some(class)) =>
                            n.cmp_class(&class),
                        _ => false,
                    }
                };
                node.modify(
                    |n| n.set_attr(attr.as_string_rep(), attr.clone()),
                    p
                )
            }
            Instruction::Then(f, s) =>
                Self::attr_instruction(Self::attr_instruction(node, *f), *s)
        }
    }

    // HACK: `Panic`king is done instead of having a type-level error handling, to make it
    // easier to implement
    // TODO: Make type-level error handling
    pub fn build(self, ui: Html) -> Html {
        let (node, text, attr) = self.instruction();
        let mut root = ui;
        for i in 0..self.count {
            if let Some((_, inst)) = node.iter().find(|(j, _)| *j == i) {
                root = Self::node_instruction(root, inst.clone());
            }
            if let Some((_, inst)) = text.iter().find(|(j, _)| *j == i) {
                root = Self::text_instruction(root, inst.clone());
            }
            if let Some((_, inst)) = attr.iter().find(|(j, _)| *j == i) {
                root = Self::attr_instruction(root, inst.clone());
            }
        }
        root
    }

}

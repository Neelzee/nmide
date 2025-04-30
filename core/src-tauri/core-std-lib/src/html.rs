//! *Html
//!
//! An Html type is an representation of an HTML-Element.

use crate::attrs::Attr;
use crate::instruction::inst::Instruction;
use core_macros::define_html;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

define_html!(
    attr_type = Attr,
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS, PartialOrd, Eq, Hash)]
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
    Main,
    TextArea
);

impl Html {
    pub fn get_by_id(ui: Html, id: String) -> Option<Self> {
        if ui.cmp_id(&id) {
            Some(ui)
        } else {
            ui.kids()
                .into_iter()
                .find_map(|h| Self::get_by_id(h, id.clone()))
        }
    }

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

    pub fn rem_attr(self, attr: Attr) -> Self {
        let attrs = if attr.is_empty() {
            self.attrs()
                .into_iter()
                .filter(|a| !a.is(attr.as_string_rep()))
                .collect()
        } else {
            self.attrs()
                .into_iter()
                .map(|a| if a == attr { a.remove(attr.value()) } else { a })
                .collect()
        };

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
            let new_kids = self
                .kids()
                .into_iter()
                .map(|k| k.modify(f.clone(), p.clone()))
                .collect();
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
    node: Instruction<Html>,
    text: Instruction<String>,
    attr: Instruction<Attr>,
}

impl Default for UIInstructionBuilder {
    fn default() -> Self {
        Self {
            node: Instruction::NoOp,
            text: Instruction::NoOp,
            attr: Instruction::NoOp,
        }
    }
}

impl UIInstructionBuilder {
    pub(crate) fn new(inst: (Instruction<Html>, Instruction<String>, Instruction<Attr>)) -> Self {
        let (node, text, attr) = inst;
        Self { node, text, attr }
    }

    pub fn instruction(&self) -> (Instruction<Html>, Instruction<String>, Instruction<Attr>) {
        (self.node.clone(), self.text.clone(), self.attr.clone())
    }

    pub fn set_text<S: ToString>(self, id: Option<S>, text: String) -> Self {
        Self {
            text: self.text.combine(Instruction::Add(
                id.map(|s| s.to_string()).unwrap_or_default(),
                text,
            )),
            ..self
        }
    }

    pub fn add_node<S: ToString>(self, ui: Html, id: Option<S>) -> Self {
        Self {
            node: self.node.combine(Instruction::Add(
                id.map(|s| s.to_string()).unwrap_or_default(),
                ui,
            )),
            ..self
        }
    }

    pub fn add_nodes<S: ToString>(self, uis: Vec<Html>, id: Option<S>) -> Self {
        let mut new = self;
        let id = id.map(|s| s.to_string()).unwrap_or_default();
        for ui in uis {
            new = Self {
                node: new.node.combine(Instruction::Add(id.clone(), ui)),
                ..new
            }
        }
        new
    }

    pub fn rem_node(self, id: String) -> Self {
        Self {
            node: self.node.combine(Instruction::Rem(id, Html::Div())),
            ..self
        }
    }

    pub fn add_attr(self, id: String, attr: Attr) -> Self {
        Self {
            attr: self.attr.combine(Instruction::Add(id, attr)),
            ..self
        }
    }

    pub fn rem_attr(self, attr: Attr, id: String) -> Self {
        Self {
            attr: self.attr.combine(Instruction::Rem(id, attr)),
            ..self
        }
    }

    fn node_instruction(node: Html, inst: Instruction<Html>) -> Html {
        match inst {
            Instruction::NoOp => node,
            // ADD
            Instruction::Add(id, child) if id == "" => node.adopt(child),
            Instruction::Add(id, child) => {
                node.modify(|n| n.adopt(child.clone()), |n| n.cmp_id(&id))
            }
            // REM
            Instruction::Rem(id, _) => {
                let child = |n: &Html| n.cmp_id(&id);
                let parent = |p: &Html| p.kids().iter().any(&child);
                node.modify(
                    |p| {
                        let kids = p.kids().into_iter().filter(child).collect();
                        p.replace_kids(kids)
                    },
                    parent,
                )
            }
            Instruction::Then(f, s) => Self::node_instruction(Self::node_instruction(node, *f), *s),
        }
    }

    fn text_instruction(node: Html, inst: Instruction<String>) -> Html {
        match inst {
            Instruction::NoOp => node,
            // ADD
            Instruction::Add(id, text) => {
                node.modify(|n| n.set_text(text.clone()), |n| n.cmp_id(&id))
            }
            // REM
            Instruction::Rem(id, _) => {
                let p = |n: &Html| n.cmp_id(&id);
                node.modify(|n| n.set_text(""), p)
            }
            Instruction::Then(f, s) => Self::text_instruction(Self::text_instruction(node, *f), *s),
        }
    }

    fn attr_instruction(node: Html, inst: Instruction<Attr>) -> Html {
        match inst {
            Instruction::NoOp => node,
            // ADD
            Instruction::Add(id, attr) => {
                node.modify(|n| n.add_attr(attr.clone()), |n| n.cmp_id(&id))
            }
            // REM
            Instruction::Rem(id, attr) => {
                let p = |n: &Html| n.cmp_id(&id);
                node.modify(|n| n.rem_attr(attr.clone()), p)
            }
            Instruction::Then(f, s) => Self::attr_instruction(Self::attr_instruction(node, *f), *s),
        }
    }

    // HACK: `Panic`king is done instead of having a type-level error handling, to make it
    // easier to implement
    // TODO: Make type-level error handling
    pub fn build(self, ui: Html) -> Html {
        let (node, text, attr) = self.instruction();
        Self::attr_instruction(
            Self::text_instruction(Self::node_instruction(ui, node), text),
            attr,
        )
    }
}

//! *Html
//!
//! An Html type is an representation of an HTML-Element.

use crate::attrs::Attr;
use core_macros::define_html;
use serde;

define_html!(
    Div, P, H1, H2, H3, H4, H5, H6, Span, Section, Article, Aside, Audio, B, Br, Button, Code, Em,
    Fieldset, Form, Img, Input, Label, Link, Li, Menu, Nav, Ol, Option, Select, Style, Svg, Table,
    Td, Th, Tr, Ul, Video, Frag, Script, Tbody, Main
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
        F: Fn(Self) -> Self + Copy,
        P: Fn(&Self) -> bool + Copy,
    {
        if p(&self) {
            f(self)
        } else {
            let new_kids = self.kids().into_iter().map(|k| k.modify(f, p)).collect();
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

// NOTE: This is at minimum, a semigroup, could be argued its a monoid and also a group.
#[derive(Default)]
pub(crate) enum UIInstruction {
    #[default]
    NoOp,
    AddAttr {
        field: String,
        attr: Attr,
    },
    SetAttr {
        field: String,
        attr: Attr,
    },
    RemAttr {
        field: String,
    },
    Add {
        ui: Html,
        id: Option<String>,
        class: Option<String>,
    },
    Rem {
        id: Option<String>,
        class: Option<String>,
    },
    AddAttrPred {
        field: String,
        id: Option<String>,
        class: Option<String>,
        attr: Attr,
    },
    SetAttrPred {
        field: String,
        id: Option<String>,
        class: Option<String>,
        attr: Attr,
    },
    RemAttrPred {
        id: Option<String>,
        class: Option<String>,
        field: String,
    },
    Then {
        fst: Box<UIInstruction>,
        snd: Box<UIInstruction>,
    },
}

impl UIInstruction {
    pub fn combine(self, other: Self) -> Self {
        match (&self, &other) {
            (Self::NoOp, _) => other,
            (_, Self::NoOp) => self,
            (Self::Add { ui, .. }, Self::Rem { id, class })
            | (Self::Rem { id, class }, Self::Add { ui, .. })
                if (id.is_some() || class.is_some()) =>
            {
                let is_removed = if let Some(id) = id {
                    ui.cmp_id(id)
                } else if let Some(class) = class {
                    ui.cmp_class(class)
                } else {
                    false
                };
                if is_removed {
                    Self::NoOp
                } else {
                    Self::Then {
                        fst: Box::new(self),
                        snd: Box::new(other),
                    }
                }
            }
            _ => Self::Then {
                fst: Box::new(self),
                snd: Box::new(other),
            },
        }
    }
}

#[derive(Default)]
pub struct UIInstructionBuilder(UIInstruction);

impl UIInstructionBuilder {
    pub(crate) fn new(ins: UIInstruction) -> Self {
        Self(ins)
    }

    pub(crate) fn instruction(self) -> UIInstruction {
        self.0
    }

    pub fn add_node(self, ui: Html, id: Option<String>, class: Option<String>) -> Self {
        Self::new(self.0.combine(UIInstruction::Add { ui, id, class }))
    }

    pub fn rem_node(self, id: Option<String>, class: Option<String>) -> Self {
        Self::new(self.0.combine(UIInstruction::Rem { id, class }))
    }

    pub fn root_add(self, field: String, attr: Attr) -> Self {
        Self::new(self.0.combine(UIInstruction::AddAttr { field, attr }))
    }

    pub fn add_attr(
        self,
        field: String,
        id: Option<String>,
        class: Option<String>,
        attr: Attr,
    ) -> Self {
        Self::new(self.0.combine(UIInstruction::AddAttrPred {
            field,
            id,
            class,
            attr,
        }))
    }

    pub fn root_remove(self, field: String) -> Self {
        Self::new(self.0.combine(UIInstruction::RemAttr { field }))
    }

    pub fn rem_attr(self, field: String, id: Option<String>, class: Option<String>) -> Self {
        Self::new(
            self.0
                .combine(UIInstruction::RemAttrPred { field, id, class }),
        )
    }

    pub fn root_set(self, field: String, attr: Attr) -> Self {
        Self::new(self.0.combine(UIInstruction::SetAttr { field, attr }))
    }

    pub fn set_attr(
        self,
        field: String,
        id: Option<String>,
        class: Option<String>,
        attr: Attr,
    ) -> Self {
        Self::new(self.0.combine(UIInstruction::SetAttrPred {
            field,
            id,
            class,
            attr,
        }))
    }

    // HACK: `Panic`king is done instead of having a type-level error handling, to make it
    // easier to implement
    fn _build(ins: UIInstruction, ui: Html) -> Html {
        match ins {
            UIInstruction::NoOp => ui,
            UIInstruction::AddAttr { field, attr } => {
                if ui.has_attr(&field) {
                    panic!("UI element already has attribute: {field}");
                }
                ui.add_attr(attr)
            }
            UIInstruction::SetAttr { field, attr } => {
                if !ui.has_attr(&field) {
                    panic!("UI element does not have attribute: {field}, to set");
                }
                ui.set_attr(&field, attr)
            }
            UIInstruction::RemAttr { field } => {
                if !ui.has_attr(&field) {
                    panic!("UI element does not have attribute: {field}, to remove");
                }
                ui.rem_attr(&field)
            }
            UIInstruction::Then { fst, snd } => {
                let fst_ins = *fst;
                let snd_ins = *snd;
                Self::_build(snd_ins, Self::_build(fst_ins, ui))
            }
            UIInstruction::Add { ui: kid, id, class } => {
                if let Some(id) = id {
                    return ui.modify(|h| h.adopt(kid.clone()), |h| h.cmp_id(&id));
                }
                if let Some(class) = class {
                    return ui.modify(|h| h.adopt(kid.clone()), |h| h.cmp_class(&class));
                }
                ui.adopt(kid)
            }
            UIInstruction::Rem { id, class } => {
                if let Some(id) = id {
                    return ui.modify(
                        |h| {
                            let new_kids =
                                h.kids().into_iter().filter(|k| !k.cmp_id(&id)).collect();
                            h.replace_kids(new_kids)
                        },
                        |h| h.kids().iter().any(|k| k.cmp_id(&id)),
                    );
                }
                if let Some(class) = class {
                    return ui.modify(
                        |h| {
                            let new_kids = h
                                .kids()
                                .into_iter()
                                .filter(|k| !k.cmp_class(&class))
                                .collect();
                            h.replace_kids(new_kids)
                        },
                        |h| h.kids().iter().any(|k| k.cmp_class(&class)),
                    );
                }
                ui
            }
            UIInstruction::AddAttrPred {
                field,
                id,
                class,
                attr,
            } => {
                if let Some(id) = id {
                    return ui.modify(|h| h.add_attr(attr.clone()), |h| h.cmp_id(&id));
                }
                if let Some(class) = class {
                    return ui.modify(|h| h.add_attr(attr.clone()), |h| h.cmp_class(&class));
                }
                ui.add_attr(attr)
            }
            UIInstruction::SetAttrPred {
                field,
                id,
                class,
                attr,
            } => {
                if let Some(id) = id {
                    return ui.modify(|h| h.set_attr(&field, attr.clone()), |h| h.cmp_id(&id));
                }
                if let Some(class) = class {
                    return ui.modify(
                        |h| h.set_attr(&field, attr.clone()),
                        |h| h.cmp_class(&class),
                    );
                }
                ui.set_attr(&field, attr)
            }
            UIInstruction::RemAttrPred { id, class, field } => {
                if let Some(id) = id {
                    return ui.modify(|h| h.rem_attr(&field), |h| h.cmp_id(&id));
                }
                if let Some(class) = class {
                    return ui.modify(|h| h.rem_attr(&field), |h| h.cmp_class(&class));
                }
                ui.rem_attr(&field)
            }
        }
    }

    // HACK: `Panic`king is done instead of having a type-level error handling, to make it
    // easier to implement
    // TODO: Make type-level error handling
    pub fn build(self, state: Html) -> Html {
        Self::_build(self.0, state)
    }
}

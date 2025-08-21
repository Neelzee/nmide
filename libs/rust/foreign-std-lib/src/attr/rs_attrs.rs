//! RAttr

// TODO: Add doc-string

use crate::event::rs_event::REvent;
use abi_stable::{StableAbi, std_types::RString};
use core_std_lib::attrs::Attr;
use std::{mem::ManuallyDrop, str::FromStr};

// TODO: Add doc-string
#[repr(C)]
#[derive(StableAbi)]
pub struct RAttr {
    pub(crate) kind: RAttrKind,
    pub(crate) val: RAttrUnion,
}

impl std::fmt::Debug for RAttr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RAttr").field("kind", &self.kind).finish()
    }
}

impl Clone for RAttr {
    fn clone(&self) -> Self {
        match self.kind {
            RAttrKind::Id | RAttrKind::Class | RAttrKind::Src => Self {
                kind: self.kind.clone(),
                val: unsafe {
                    RAttrUnion {
                        _str: self.val._str.clone(),
                    }
                },
            },
            RAttrKind::Style => todo!(),
            RAttrKind::Type => todo!(),
            RAttrKind::Checked => todo!(),
            RAttrKind::OnClick => Self {
                kind: self.kind.clone(),
                val: unsafe {
                    RAttrUnion {
                        _msg: self.val._msg.clone(),
                    }
                },
            },
            RAttrKind::OnInput => todo!(),
            RAttrKind::EmitInput => todo!(),
        }
    }
}

impl RAttr {
    // TODO: Add doc-test
    /// Gets a possible [`RString`] value from the RAttr.
    /// This will only be `Some` if the RAttr is of type `Id`, `Class` or `Style`
    ///
    /// [`RString`]: https://docs.rs/abi_stable/latest/abi_stable/std_types/struct.RString.html
    pub fn str(&self) -> Option<ManuallyDrop<RString>> {
        // TODO: Ensure this holds for all Attributes.
        match self.kind {
            RAttrKind::Id | RAttrKind::Class | RAttrKind::Style => {
                Some(unsafe { self.val._str.clone() })
            }
            _ => None,
        }
    }

    // TODO: Add doc-test
    /// Gets a possible [`Bool`] value from the RAttr.
    /// This will only be [`Some`] if the RAttr is of type `Checked`.
    pub fn bool(&self) -> Option<bool> {
        match self.kind {
            RAttrKind::Checked => Some(unsafe { self.val._bool }),
            _ => None,
        }
    }

    // TODO: Add doc-test
    /// Gets a possible [`REvent`] value from the RAttr.
    /// This will only be [`Some`] if the RAttr is of type `OnClick` or `OnInput`
    ///
    /// [`REvent`]: ../msg/rmsg.rs
    pub fn event(&self) -> Option<ManuallyDrop<REvent>> {
        match self.kind {
            RAttrKind::OnClick => Some(unsafe { self.val._msg.clone() }),
            _ => None,
        }
    }

    // TODO: Add doc-test
    /// Creates a new `Src` RAttr
    pub fn new_src(src: RString) -> Self {
        Self {
            kind: RAttrKind::Src,
            val: RAttrUnion {
                _str: ManuallyDrop::new(src),
            },
        }
    }

    // TODO: Add doc-test
    /// Creates a new `ID` RAttr
    pub fn new_id<S: ToString>(id: S) -> Self {
        Self {
            kind: RAttrKind::Id,
            val: RAttrUnion {
                _str: ManuallyDrop::new(RString::from(id.to_string())),
            },
        }
    }

    // TODO: Add doc-test
    /// Creates a new `Class` RAttr
    pub fn new_class(class: RString) -> Self {
        Self {
            kind: RAttrKind::Class,
            val: RAttrUnion {
                _str: ManuallyDrop::new(class),
            },
        }
    }

    // TODO: Add doc-test
    /// Creates a new `OnClick` RAttr
    pub fn new_click(rmsg: REvent) -> Self {
        Self {
            kind: RAttrKind::OnClick,
            val: RAttrUnion {
                _msg: ManuallyDrop::new(rmsg),
            },
        }
    }

    // TODO: Add doc-test
    /// Creates a new `OnInput` RAttr
    pub fn new_on_input(rmsg: REvent) -> Self {
        Self {
            kind: RAttrKind::OnInput,
            val: RAttrUnion {
                _msg: ManuallyDrop::new(rmsg),
            },
        }
    }

    // TODO: Add doc-test
    /// Creates a new `EmitInput` RAttr
    pub fn new_emit_input(rmsg: RString) -> Self {
        Self {
            kind: RAttrKind::EmitInput,
            val: RAttrUnion {
                _str: ManuallyDrop::new(rmsg),
            },
        }
    }
}

#[repr(u8)]
#[derive(StableAbi, Clone, Debug)]
pub enum RAttrKind {
    Id,
    Class,
    Style,
    Type,
    Checked,
    OnClick,
    OnInput,
    EmitInput,
    Src,
}

#[repr(C)]
#[derive(StableAbi)]
pub union RAttrUnion {
    _str: ManuallyDrop<RString>,
    _bool: bool,
    _msg: ManuallyDrop<REvent>,
}

impl From<Attr> for RAttr {
    fn from(value: Attr) -> Self {
        match value {
            Attr::Id(s) => Self::new_id(RString::from_str(&s).unwrap_or_default()),
            Attr::Class(s) => Self::new_class(RString::from_str(&s).unwrap_or_default()),
            Attr::Click(event) => Self::new_click(event.into()),
            _ => unimplemented!(),
        }
    }
}

impl RAttr {
    pub fn to_attr(self) -> Attr {
        match self.kind {
            RAttrKind::Id => Attr::Id(self.str().unwrap().as_str().to_string()),
            RAttrKind::Class => Attr::Class(self.str().unwrap().as_str().to_string()),
            RAttrKind::Style => todo!(),
            RAttrKind::Type => todo!(),
            RAttrKind::Checked => todo!(),
            RAttrKind::OnClick => {
                let evt = self.event().unwrap();
                let event = evt.to_event();
                Attr::Click(event)
            }
            RAttrKind::OnInput => todo!(),
            RAttrKind::EmitInput => todo!(),
            RAttrKind::Src => todo!(),
        }
    }
}

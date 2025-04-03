//! RAttr

// TODO: Add doc-string

use crate::msg::rmsg::RMsg;
use abi_stable::{std_types::RString, StableAbi};
use std::mem::ManuallyDrop;

// TODO: Add doc-string
#[repr(C)]
#[derive(StableAbi)]
pub struct RAttr {
    pub(crate) kind: RAttrKind,
    pub(crate) val: RAttrUnion,
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
            RAttrKind::OnClick => todo!(),
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
            RAttrKind::Checked => Some(unsafe { self.val._bool.clone() }),
            _ => None,
        }
    }

    // TODO: Add doc-test
    /// Gets a possible [`RMsg`] value from the RAttr.
    /// This will only be [`Some`] if the RAttr is of type `OnClick` or `OnInput`
    ///
    /// [`RMsg`]: ../msg/rmsg.rs
    pub fn msg(&self) -> Option<ManuallyDrop<RMsg>> {
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
    pub fn new_id(id: RString) -> Self {
        Self {
            kind: RAttrKind::Id,
            val: RAttrUnion {
                _str: ManuallyDrop::new(id),
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
    pub fn new_click(rmsg: RMsg) -> Self {
        Self {
            kind: RAttrKind::OnClick,
            val: RAttrUnion {
                _msg: ManuallyDrop::new(rmsg),
            },
        }
    }

    // TODO: Add doc-test
    /// Creates a new `OnInput` RAttr
    pub fn new_on_input(rmsg: RMsg) -> Self {
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
#[derive(StableAbi, Clone)]
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
    _msg: ManuallyDrop<RMsg>,
}

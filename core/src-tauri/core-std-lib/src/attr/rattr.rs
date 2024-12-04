use crate::msg::rmsg::RMsg;
use abi_stable::{std_types::RString, StableAbi};
use std::mem::ManuallyDrop;

#[repr(C)]
#[derive(StableAbi)]
pub struct RAttr {
    pub(crate) kind: RAttrKind,
    pub(crate) val: RAttrUnion,
}

impl RAttr {
    pub fn str(&self) -> Option<ManuallyDrop<RString>> {
        match self.kind {
            RAttrKind::Id | RAttrKind::Class | RAttrKind::Style => {
                Some(unsafe { self.val._str.clone() })
            }
            _ => None,
        }
    }

    pub fn bool(&self) -> Option<bool> {
        match self.kind {
            RAttrKind::Checked => Some(unsafe { self.val._bool.clone() }),
            _ => None,
        }
    }

    pub fn msg(&self) -> Option<ManuallyDrop<RMsg>> {
        match self.kind {
            RAttrKind::OnClick => Some(unsafe { self.val._msg.clone() }),
            _ => None,
        }
    }

    pub fn new_src(src: RString) -> Self {
        Self {
            kind: RAttrKind::Src,
            val: RAttrUnion {
                _str: ManuallyDrop::new(src),
            },
        }
    }

    pub fn new_id(id: RString) -> Self {
        Self {
            kind: RAttrKind::Id,
            val: RAttrUnion {
                _str: ManuallyDrop::new(id),
            },
        }
    }

    pub fn new_class(class: RString) -> Self {
        Self {
            kind: RAttrKind::Class,
            val: RAttrUnion {
                _str: ManuallyDrop::new(class),
            },
        }
    }

    pub fn new_click(rmsg: RMsg) -> Self {
        Self {
            kind: RAttrKind::OnClick,
            val: RAttrUnion {
                _msg: ManuallyDrop::new(rmsg),
            },
        }
    }

    pub fn new_on_input(rmsg: RMsg) -> Self {
        Self {
            kind: RAttrKind::OnInput,
            val: RAttrUnion {
                _msg: ManuallyDrop::new(rmsg),
            },
        }
    }

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
#[derive(StableAbi)]
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

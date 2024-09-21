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

    pub fn msg(&self) -> Option<ManuallyDrop<RMsg>> {
        match self.kind {
            RAttrKind::OnClick => Some(unsafe { self.val._msg.clone() }),
            _ => None,
        }
    }
}

#[repr(u8)]
#[derive(StableAbi)]
pub enum RAttrKind {
    Id,
    Class,
    Style,
    OnClick,
}

#[repr(C)]
#[derive(StableAbi)]
pub union RAttrUnion {
    _str: ManuallyDrop<RString>,
    _msg: ManuallyDrop<RMsg>,
}

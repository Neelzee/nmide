use crate::msg::rmsg::RMsg;
use abi_stable::{std_types::RString, StableAbi};
use std::mem::ManuallyDrop;

#[repr(C)]
#[derive(StableAbi)]
pub struct RAttr {
    kind: RAttrKind,
    val: ManuallyDrop<RString>,
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

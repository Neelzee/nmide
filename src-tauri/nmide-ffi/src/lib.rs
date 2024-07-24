use ::safer_ffi::prelude::*;

#[derive_ReprC]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Element {
    Div,
    P,
    Text,
    Span,
    Img,
    Button,
}

#[derive_ReprC]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Html {
    kind: Element,
    attr: [Option<u8>; 10],
    kids: [Box<Html>; 10],
}

#[ffi_export]
fn div() -> Html {
    Html {
        kind: Element::Div,
        attr: [],
        kids: [],
    }
}

#[cfg(feature = "headers")]
pub fn generate_headers() -> ::std::io::Result<()> {
    ::safer_ffi::headers::builder()
        .to_file("nmide.h")?
        .generate()
}

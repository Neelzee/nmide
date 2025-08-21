//! CHtml
//!
//! Work-in-progress module for Rust-representations of the HTML-system, from C.
//!
//! When this is completed, it would allow for more Plugins in different languages.

use abi_stable::std_types::{ROption, RString, RVec, string::FromUtf8Error};
use core::slice;

macro_rules! chtmlkind {
    ( $( $name:ident ),* ) => {
        #[stabby::stabby]
        #[derive(Clone)]
        #[repr(u8)]
        pub enum CHtmlKind {
            $(
                $name,
            )*
        }

        /*
        impl CHtmlKind {
            pub fn to_rhtml(&self) -> RHtmlKind {
                match self {
                    $(
                        CHtmlKind::$name => RHtmlKind::$name,
                        )*
                }
            }
        }
        */
    };
}

chtmlkind!(
    Div, P, H1, H2, H3, H4, H5, H6, Span, Section, Article, Aside, Audio, B, Br, Button, Code, Em,
    Fieldset, Form, Img, Input, Label, Link, Li, Menu, Nav, Ol, Option, Select, Style, Svg, Table,
    Tbody, Td, Th, Tr, Ul, Video, Frag, Script, Text
);

/*
macro_rules! htmlkinds {
    Div, P, H1, H2, H3, H4, H5, H6, Span, Section, Article, Aside, Audio, B, Br, Button, Code, Em,
    Fieldset, Form, Img, Input, Label, Link, Li, Menu, Nav, Ol, Option, Select, Style, Svg, Table,
    Tbody, Td, Th, Tr, Ul, Video, Frag, Script, Text
};
*/

#[stabby::stabby]
#[derive(Clone)]
#[repr(C)]
pub struct CHtml {
    kind: CHtmlKind,
    kids: *const CHtml,
    kids_len: usize,
    text: *const u8,
    text_len: usize,
}

impl CHtml {
    /*
    pub fn to_rhtml(&self) -> Result<RHtml, FromUtf8Error> {
            let kids = if self.kids.is_null() {
                RVec::new()
            } else {
                RVec::from_iter(
                    unsafe { slice::from_raw_parts(self.kids, self.kids_len) }
                        .into_iter()
                        .filter_map(|r| r.to_rhtml().ok()),
                )
            };
            Ok(RHtml {
                kind: self.kind.to_rhtml(),
                kids,
                text: if !self.text.is_null() {
                    ROption::RSome(RString::from_utf8(unsafe {
                        slice::from_raw_parts(self.text, self.text_len)
                    })?)
                } else {
                    ROption::RNone
                },
                attrs: RVec::new(),
            })
        }
    */
}

//! CHtml
//!
//! Work-in-progress module for Rust-representations of the HTML-system, from C.
//!
//! When this is completed, it would allow for more Plugins in different languages.

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

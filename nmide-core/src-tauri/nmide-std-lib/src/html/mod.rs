pub mod chtml;
pub mod rhtml;
pub mod thtml {
    use serde::{Deserialize, Serialize};
    use ts_rs::TS;

    use crate::attr::tattr::TAttr;

    macro_rules! thtmlkind {
        ( $( $name:ident ),* ) => {
            #[derive(Serialize, Deserialize, TS)]
            #[ts(export)]
            pub enum THtmlKind {
                $(
                    $name,
                )*
            }
        };
    }

    thtmlkind!(
        Div, P, H1, H2, H3, H4, H5, H6, Span, Section, Article, Aside, Audio, B, Br, Button, Code,
        Em, Fieldset, Form, Img, Input, Label, Link, Li, Menu, Nav, Ol, Option, Select, Style, Svg,
        Table, Td, Th, Ul, Video, Frag, Text
    );

    #[derive(Serialize, Deserialize, TS)]
    #[ts(export)]
    pub struct THtml {
        kind: THtmlKind,
        kids: Vec<THtml>,
        text: Option<String>,
        attrs: Vec<TAttr>,
    }
}

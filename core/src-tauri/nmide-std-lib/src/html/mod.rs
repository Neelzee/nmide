pub mod chtml;
pub mod rhtml;
pub mod thtml {
    use crate::{
        attr::tattr::TAttr,
        html::rhtml::{RHtml, RHtmlKind},
    };
    use abi_stable::std_types::ROption;
    use serde::{Deserialize, Serialize};
    use ts_rs::TS;

    macro_rules! thtmlkind {
        ( $( $name:ident ),* ) => {
            #[derive(Serialize, Deserialize, TS)]
            #[serde(rename_all = "camelCase")]
            #[ts(export)]
            pub enum THtmlKind {
                $(
                    $name,
                )*
            }
            impl From<RHtmlKind> for THtmlKind {
                fn from(value: RHtmlKind) -> Self {
                    match value {
                        $(
                            RHtmlKind::$name => Self::$name,
                        )*
                    }
                }
            }
        };
    }

    thtmlkind!(
        Div, P, H1, H2, H3, H4, H5, H6, Span, Section, Article, Aside, Audio, B, Br, Button, Code,
        Em, Fieldset, Form, Img, Input, Label, Link, Li, Menu, Nav, Ol, Option, Select, Style, Svg,
        Table, Td, Th, Tr, Ul, Video, Frag, Text, Script, Tbody
    );

    #[derive(Serialize, Deserialize, TS)]
    #[serde(rename_all = "camelCase")]
    #[ts(export)]
    pub struct THtml {
        kind: THtmlKind,
        kids: Vec<THtml>,
        text: Option<String>,
        attrs: Vec<TAttr>,
    }

    impl From<&RHtml> for THtml {
        fn from(value: &RHtml) -> Self {
            Self {
                kind: value.kind.clone().into(),
                kids: value.kids.iter().map(|k| k.into()).collect(),
                text: match value.text.clone() {
                    ROption::RSome(txt) => Some(txt.to_string()),
                    ROption::RNone => None,
                },
                attrs: value.attrs.iter().map(|k| k.into()).collect(),
            }
        }
    }

    impl From<RHtml> for THtml {
        fn from(value: RHtml) -> Self {
            Self {
                kind: value.kind.into(),
                kids: value.kids.iter().map(|k| k.into()).collect(),
                text: match value.text {
                    ROption::RSome(txt) => Some(txt.to_string()),
                    ROption::RNone => None,
                },
                attrs: value.attrs.iter().map(|k| k.into()).collect(),
            }
        }
    }
}

use crate::attr::ts_attrs::TAttr;
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
        };
    }

thtmlkind!(
    Div, P, H1, H2, H3, H4, H5, H6, Span, Section, Article, Aside, Audio, B, Br, Button, Code, Em,
    Fieldset, Form, Img, Input, Label, Link, Li, Menu, Nav, Ol, Option, Select, Style, Svg, Table,
    Td, Th, Tr, Ul, Video, Frag, Text, Script, Tbody, Main
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

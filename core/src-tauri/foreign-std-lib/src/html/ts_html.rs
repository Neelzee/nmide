use crate::attr::ts_attrs::TAttr;
use core_macros::define_html;
use core_std_lib::html::UIInstruction;
use serde::{self, Deserialize, Serialize};
use ts_rs::TS;

define_html!(
    attr_type = TAttr,
    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, TS)]
    #[serde(rename_all = "camelCase")]
    #[ts(export)]
    Div,
    P,
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
    Span,
    Section,
    Article,
    Aside,
    Audio,
    B,
    Br,
    Button,
    Code,
    Em,
    Fieldset,
    Form,
    Img,
    Input,
    Label,
    Link,
    Li,
    Menu,
    Nav,
    Ol,
    Option,
    Select,
    Style,
    Svg,
    Table,
    Td,
    Th,
    Tr,
    Ul,
    Video,
    Frag,
    Script,
    Tbody,
    Main
);

#[derive(Default, Deserialize, Serialize, Clone, TS)]
#[ts(export, export_to = "Html.ts")]
pub struct TUIInstruction {
    #[ts(inline)]
    op: UIInstruction,
}

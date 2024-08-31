#[macro_export]
macro_rules! define_html {
    ( $( $name:ident ),* ) => {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
        pub enum Html {
            $(
                $name { kids: Vec<Html>, attrs: Vec<Attr> },
            )*
            Text(String),
        }

        impl Html {

            pub fn cast_html(self, target: Self) -> Self {
                match self {
                    // Match each variant and cast to the target variant if possible
                    $(
                        Html::$name { kids, attrs } => {
                            match target {
                                Html::$name { .. } => Html::$name { kids, attrs }, // Cast to the same type
                                _ => target._cast_html(kids, attrs), // Cast to the target type
                            }
                        },
                    )*
                    Html::Text(text) => Html::Text(text), // Text remains Text
                }
            }

            // Helper method to create a target variant
            fn _cast_html(self, kids: Vec<Html>, attrs: Vec<Attr>) -> Self {
                match self {
                    $(
                        Html::$name { .. } => Html::$name { kids, attrs },
                    )*
                    Html::Text(text) => Html::Text(text), // Text remains Text
                }
            }

            $(
            #[allow(non_snake_case)]
            pub fn $name() -> Self {
                Self::$name { kids: Vec::new(), attrs: Vec::new() }
            }
            )*

            /// Returns a copy of the kids
            pub fn kids(&self) -> Vec<Html> {
                match self {
                $(
                    Self::$name { kids, attrs } => kids.clone(),
                )*
                    Self::Text(_) => Vec::new(),
                }
            }

            /// Returns a copy of the attributes
            pub fn attrs(&self) -> Vec<Attr> {
                match self {
                $(
                    Self::$name { kids: _, attrs } => attrs.clone(),
                )*
                    Self::Text(_) => Vec::new(),
                }
            }

            /// Adds the given Html node to itself
            pub fn adopt(self, kid: Html) -> Self {
                match self {
                $(
                    Self::$name { mut kids, attrs } => {
                        kids.push(kid);
                        Self::$name { kids, attrs }
                    },
                )*
                    html @ _ => html,
                }
            }

            pub fn replace_kids(self, new_kids: Vec<Html>) -> Self {
                match self {
                $(
                    Self::$name { kids: _, attrs } => Self::$name { kids: new_kids, attrs },
                )*
                    html @ _ => html,
                }
            }

            pub fn to_ts_html_kind(&self) -> TSHtmlKind {
                match self {
                    $(
                        Self::$name { .. } => TSHtmlKind::$name,
                    )*
                        Self::Text(_) => TSHtmlKind::Text,
                }
            }
        }

        #[derive(
            Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord, TS,
        )]
        #[ts(export, export_to = "../../../src/bindings/TSHtmlKind.ts")]
        pub enum TSHtmlKind {
            $(
                $name,
            )*
            Text,
        }

        impl TSHtmlKind {
            pub fn to_html(&self) -> Html {
                match self {
                    $(
                        Self::$name => Html::$name(),
                    )*
                        Self::Text => Html::Text(String::new()),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! css {
    ($( $style:ident $e:expr $(; $unit:ident)? ),*) => {
         vec![$(Style::$style($e.to_string(), $(Unit::$unit)?)),*]
    };
}

#[macro_export]
macro_rules! attr {
    ( $attr:ident $v:expr ) => {
        Attr::$attr($v.to_string())
    };

    ( $attr:ident => $v:expr ) => {
        Attr::$attr(Msg::from_string($v))
    };
}

#[macro_export]
macro_rules! attrs {
    ($( $attr:ident $v:expr ),*) => {
         vec![$(Attr::$attr($v.to_string())),*]
    };
}

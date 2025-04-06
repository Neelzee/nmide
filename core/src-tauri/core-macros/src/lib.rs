// TODO: Add docs
// TODO: See if we can refactor this to be better
#[macro_export]
macro_rules! define_html {
    ( $( $name:ident ),* ) => {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
        pub enum Html {
            $(
                $name { kids: Vec<Html>, attrs: Vec<Attr> },
            )*
            Text(String),
        }

        impl Html {
            pub fn shallow_clone(&self) -> Self {
                match self {
                    $(
                        Html::$name { .. } => Html::$name(),
                    )*
                    Html::Text(_) => Html::Text(String::new()),
                }
            }

            pub fn cast_html(self, target: Self) -> Self {
                match self {
                    $(
                        Html::$name { kids, attrs } => {
                            match target {
                                Html::$name { .. } => Html::$name { kids, attrs },
                                _ => target._cast_html(kids, attrs),
                            }
                        },
                    )*
                    Html::Text(text) => Html::Text(text),
                }
            }

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
            /// Creates empty $name
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

            pub fn set_attrs(self, attrs: Vec<Attr>) -> Self {
                match self {
                $(
                    Self::$name { attrs: _, kids } => Self::$name { attrs, kids },
                )*
                    Self::Text(_) => Self::P { kids: vec![self], attrs },
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

            /// Sets kids to the new supplied value
            pub fn replace_kids(self, new_kids: Vec<Html>) -> Self {
                match self {
                $(
                    Self::$name { kids: _, attrs } => Self::$name { kids: new_kids, attrs },
                )*
                    html @ _ => html,
                }
            }
        }
    };
}

// TODO: Fix
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
}

#[macro_export]
macro_rules! attrs {
    ($( $attr:ident $v:expr ),*) => {
         vec![$(Attr::$attr($v.to_string())),*]
    };
}

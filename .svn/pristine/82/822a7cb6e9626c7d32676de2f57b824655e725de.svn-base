// TODO: Add docs
// TODO: See if we can refactor this to be better
#[macro_export]
macro_rules! define_html {
    (
        attr_type = $attr:ty,
        $(#[$meta:meta])*
        $( $name:ident ),*
    ) => {
        $(#[$meta])*
        pub enum Html {
            $( $name { kids: Vec<Html>, attrs: Vec<$attr>, text: Option<String> }, )*
        }
        impl Html {
            pub fn shallow_clone(&self) -> Self {
                match self {
                    $(
                        Html::$name { .. } => Html::$name(),
                    )*
                }
            }

            pub fn cast_html(self, target: Self) -> Self {
                match self {
                    $(
                        Html::$name { kids, attrs, text } => {
                            match target {
                                Html::$name { .. } => Html::$name { kids, attrs, text },
                                _ => target._cast_html(kids, attrs, text),
                            }
                        },
                    )*
                }
            }

            fn _cast_html(self, kids: Vec<Html>, attrs: Vec<$attr>, text: Option<String>) -> Self {
                match self {
                    $(
                        Html::$name { .. } => Html::$name { kids, attrs, text },
                    )*
                }
            }

            $(
            #[allow(non_snake_case)]
            /// Creates empty $name
            pub fn $name() -> Self {
                    Self::$name { text: None, kids: Vec::new(), attrs: Vec::new() }
                }
            )*

            /// Returns a copy of the kids
            pub fn kids(&self) -> Vec<Html> {
                match self {
                $(
                    Self::$name { kids, .. } => kids.clone(),
                )*
                }
            }

            /// Returns a copy of the attributes
            pub fn attrs(&self) -> Vec<$attr> {
                match self {
                $(
                    Self::$name { attrs, .. } => attrs.clone(),
                )*
                }
            }

            pub fn set_attrs(self, attrs: Vec<$attr>) -> Self {
                match self {
                $(
                    Self::$name { attrs: _, kids, text } => Self::$name { attrs, kids, text },
                )*
                }
            }

            /// Adds the given Html node to itself
            pub fn adopt(self, kid: Html) -> Self {
                match self {
                $(
                    Self::$name { mut kids, attrs, text } => {
                        kids.push(kid);
                        Self::$name { kids, attrs, text }
                    },
                )*
                }
            }

            /// Sets kids to the new supplied value
            pub fn replace_kids(self, new_kids: Vec<Html>) -> Self {
                match self {
                $(
                    Self::$name { kids: _, attrs, text } => Self::$name { kids: new_kids, attrs, text },
                )*
                }
            }

            pub fn set_text<S: ToString>(self, s: S) -> Self {
                match self {
                $(
                    Self::$name { kids, attrs, text: _ } => {
                        Self::$name { kids, attrs, text: Some(s.to_string()) }
                    },
                )*
                }

            }

            pub fn text(&self) -> String {
                match self {
                $(
                    Self::$name { text, .. } => text.clone().unwrap_or("".to_string()),
                )*
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

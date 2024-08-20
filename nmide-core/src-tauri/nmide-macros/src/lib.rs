#[macro_export]
macro_rules! define_html {
    ( $( $name:ident ),* ) => {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
        pub enum Html {
            $(
                $name { kids: Vec<Html>, attrs: Vec<Attr> },
            )*
            Text(String),
        }

        impl Html {

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
                    Text(_) => Vec::new(),
                }
            }

            /// Returns a copy of the attributes
            pub fn attrs(&self) -> Vec<Attr> {
                match self {
                $(
                    Self::$name { kids, attrs } => attrs.clone(),
                )*
                    Text(_) => Vec::new(),
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
        }
    };
}

#[macro_export]
macro_rules! css {
    ($( $style:ident $e:expr $(; $unit:ident)? ),*) => {
         vec![$(Style::$style($e as f32, $(Unit::$unit)?)),*]
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

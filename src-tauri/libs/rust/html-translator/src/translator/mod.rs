pub(crate) mod from_html;

#[macro_export]
macro_rules! include_html {
    ( $pth:literal ) => {
        html_translator::translator::from_html::parse_html(include_str!($pth))
    }
}
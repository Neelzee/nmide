use anyhow::Result;

use crate::simple_test;

use super::{Element, Html};

#[test]
fn ffi_simple_test() -> Result<()> {
    let ch = unsafe { simple_test() };
    let html = Html::from_c(ch)?;

    let e_html = Html::new(
        Element::Div,
        vec![Html::new(
            Element::P,
            vec![Html::new(
                Element::Text("Hello, World!".to_string()),
                Vec::new(),
            )],
        )],
    );

    assert_eq!(e_html, html);

    println!("{:?}", e_html);
    println!("{:?}", html);

    Ok(())
}

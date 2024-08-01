use anyhow::Result;

use crate::{html::c_html_to_string, simple_test, CHtml};

use super::{Element, Html};

#[test]
fn ffi_simple_test() -> Result<()> {
    println!("heyo");

    let ch = unsafe { simple_test() };

    println!("{}", c_html_to_string(ch.clone()));

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

    println!("E: {:?}", e_html);
    println!("R: {:?}", html);

    assert_eq!(e_html, html);

    Ok(())
}

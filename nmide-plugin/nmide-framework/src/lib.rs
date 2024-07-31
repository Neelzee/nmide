use nmide_rust_ffi::{
    html::{Element, Html},
    CHtml,
};

#[no_mangle]
pub extern "C" fn view() -> CHtml {
    Html::new(
        Element::Div,
        vec![Html::new(
            Element::P,
            vec![Html::new(
                Element::Text("Hello, World!".to_string()),
                Vec::new(),
            )],
        )],
    )
    .to_c()
}

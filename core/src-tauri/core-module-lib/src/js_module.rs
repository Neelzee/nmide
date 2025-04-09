use serialize_to_javascript::{default_template, DefaultTemplate, Options, Serialized, Template};

#[derive(Template)]
#[default_template("js-module.js")]
pub struct JsModule {
    name: &'static str,
    #[raw]
    init: &'static str,
    #[raw]
    handler: &'static str,
}

#[test]
fn name() {
    let md = JsModule {
        name: "bar",
        init: "console.log('foobar')",
        handler: "console.log('barfoo')",
    };

    let res = md.render_default(&Options::default()).unwrap();

    println!("{res:?}");

    assert!(true != false);
}

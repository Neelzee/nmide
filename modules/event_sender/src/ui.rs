use core_std_lib::{attrs::Attr, event::Event, html::Html, state::Value};

pub fn debug_ui() -> Html {
    Html::Div()
        .add_attr(Attr::Id("debug-content".to_string()))
        .adopt(
            Html::Button()
                .set_text("Debug")
                .add_attr(Attr::Id("debug-btn".to_string()))
                .add_attr(Attr::Click(Event::new("toggle-debug", None))),
        )
        .adopt(debug_container().add_attr(Attr::Class("hide-debug".to_string())))
}

fn debug_container() -> Html {
    Html::Div()
        .add_attr(Attr::Id("debug-container".to_string()))
        .add_attr(Attr::Class("debug-hide".to_string()))
        .adopt(event_form())
}

fn event_form() -> Html {
    let str = serde_json::to_string_pretty(
        &Value::new_obj()
            .obj_add("string-field", Value::Str("foobar".to_string()))
            .obj_add(
                "list-field",
                Value::List((0..3).map(|i| Value::Int(i)).collect()),
            )
            .obj_add(
                "nested-object",
                Value::new_obj().obj_add("nested-bool-field", Value::Bool(true)),
            ),
    )
    .unwrap_or_default();

    Html::Div()
        .adopt(
            Html::Div()
                .add_attr(Attr::Id("event-kind-selector".to_string()))
                .adopt(
                    Html::Label()
                        .set_text("Event Kind: ")
                        .add_attr(Attr::Custom("for".to_string(), "event-kind".to_string())),
                )
                .adopt(
                    Html::Select()
                        .add_attr(Attr::Id("event-kind".to_string()))
                        .add_attr(Attr::Change(Event::new(
                            "event-kind-selected".to_string(),
                            None,
                        )))
                        .adopt(Html::Option().set_text("Event"))
                        .adopt(Html::Option().set_text("pre-exit"))
                        .adopt(Html::Option().set_text("post-init"))
                        .adopt(Html::Option().set_text("Core Response"))
                        .adopt(Html::Option().set_text("Dialog"))
                        .adopt(Html::Option().set_text("Dialog File")),
                ),
        )
        .adopt(
            Html::Form()
                .add_attr(Attr::Id("post-init".to_string()))
                .add_attr(Attr::Class("hide-form".to_string()))
                .adopt(
                    Html::Div()
                        .add_attr(Attr::Id("post-init-name-container".to_string()))
                        .adopt(
                            Html::Label()
                                .add_attr(Attr::Custom("for".to_string(), "event-name".to_string()))
                                .set_text("Event Name: "),
                        )
                        .adopt(
                            Html::Input()
                                .add_attr(Attr::Custom(
                                    "value".to_string(),
                                    "post-init".to_string(),
                                ))
                                .add_attr(Attr::Type("text".to_string()))
                                .add_attr(Attr::Id("post-init-name".to_string()))
                                .add_attr(Attr::Custom(
                                    "name".to_string(),
                                    "event-name".to_string(),
                                )),
                        ),
                )
                .adopt(event_form_btn()),
        )
        .adopt(
            Html::Form()
                .add_attr(Attr::Class("hide-form".to_string()))
                .add_attr(Attr::Id("pre-exit".to_string()))
                .adopt(
                    Html::Div()
                        .add_attr(Attr::Id("pre-exit-name-container".to_string()))
                        .adopt(
                            Html::Label()
                                .add_attr(Attr::Custom("for".to_string(), "event-name".to_string()))
                                .set_text("Event Name: "),
                        )
                        .adopt(
                            Html::Input()
                                .add_attr(Attr::Custom("value".to_string(), "pre-exit".to_string()))
                                .add_attr(Attr::Type("text".to_string()))
                                .add_attr(Attr::Id("pre-exit-name".to_string()))
                                .add_attr(Attr::Custom(
                                    "name".to_string(),
                                    "event-name".to_string(),
                                )),
                        ),
                )
                .adopt(event_form_btn()),
        )
        .adopt(
            Html::Form()
                .add_attr(Attr::Class("hide-form".to_string()))
                .add_attr(Attr::Id("event-form".to_string()))
                .add_attr(Attr::Class("show-form".to_string()))
                .adopt(
                    Html::Div()
                        .add_attr(Attr::Id("event-name-container".to_string()))
                        .adopt(
                            Html::Label()
                                .add_attr(Attr::Custom("for".to_string(), "event-name".to_string()))
                                .set_text("Event Name: "),
                        )
                        .adopt(
                            Html::Input()
                                .add_attr(Attr::Type("text".to_string()))
                                .add_attr(Attr::Id("event-name".to_string()))
                                .add_attr(Attr::Custom(
                                    "name".to_string(),
                                    "event-name".to_string(),
                                )),
                        ),
                )
                .adopt(
                    Html::Div()
                        .add_attr(Attr::Id("event-args-container".to_string()))
                        .adopt(
                            Html::Label()
                                .add_attr(Attr::Custom("for".to_string(), "event-args".to_string()))
                                .set_text("Args: "),
                        )
                        .adopt(
                            Html::TextArea()
                                .add_attr(Attr::Type("text".to_string()))
                                .add_attr(Attr::Id("event-args".to_string()))
                                .add_attr(Attr::Custom(
                                    "name".to_string(),
                                    "event-args".to_string(),
                                )),
                        ),
                )
                .adopt(event_form_btn()),
        )
        .adopt(
            Html::Form()
                .add_attr(Attr::Class("hide-form".to_string()))
                .add_attr(Attr::Id("core-response".to_string()))
                .adopt(
                    Html::Div()
                        .add_attr(Attr::Id("cr-event-name-container".to_string()))
                        .adopt(
                            Html::Label()
                                .add_attr(Attr::Custom("for".to_string(), "event-name".to_string()))
                                .set_text("Dialog Response Event Name: "),
                        )
                        .adopt(
                            Html::Input()
                                .add_attr(Attr::Type("text".to_string()))
                                .add_attr(Attr::Id("cr-event-name".to_string()))
                                .add_attr(Attr::Custom(
                                    "name".to_string(),
                                    "event-name".to_string(),
                                )),
                        ),
                )
                .adopt(
                    Html::Div()
                        .add_attr(Attr::Id("cr-event-args-container".to_string()))
                        .adopt(
                            Html::Label()
                                .add_attr(Attr::Custom("for".to_string(), "event-args".to_string()))
                                .set_text("Args: "),
                        )
                        .adopt(
                            Html::TextArea()
                                .add_attr(Attr::Type("text".to_string()))
                                .add_attr(Attr::Id("event-args".to_string()))
                                .add_attr(Attr::Custom(
                                    "name".to_string(),
                                    "event-args".to_string(),
                                )),
                        ),
                )
                .adopt(event_form_btn()),
        )
        .adopt(
            Html::Form()
                .add_attr(Attr::Class("hide-form".to_string()))
                .add_attr(Attr::Id("dialog-file".to_string()))
                .adopt(
                    Html::Div()
                        .add_attr(Attr::Id("dialog-file-event-name-container".to_string()))
                        .adopt(
                            Html::Label()
                                .add_attr(Attr::Custom("for".to_string(), "event-name".to_string()))
                                .set_text("Dialog File Response Event Name: "),
                        )
                        .adopt(
                            Html::Input()
                                .add_attr(Attr::Type("text".to_string()))
                                .add_attr(Attr::Id("dialog-file-event-name".to_string()))
                                .add_attr(Attr::Custom(
                                    "name".to_string(),
                                    "event-name".to_string(),
                                )),
                        ),
                )
                .adopt(
                    Html::Div()
                        .add_attr(Attr::Id("cr-event-args-container".to_string()))
                        .adopt(
                            Html::Label()
                                .add_attr(Attr::Custom("for".to_string(), "event-args".to_string()))
                                .set_text("Args: "),
                        )
                        .adopt(
                            Html::TextArea()
                                .add_attr(Attr::Type("text".to_string()))
                                .add_attr(Attr::Id("event-args".to_string()))
                                .add_attr(Attr::Custom(
                                    "name".to_string(),
                                    "event-args".to_string(),
                                )),
                        ),
                )
                .adopt(event_form_btn()),
        )
        .adopt(
            Html::Form()
                .add_attr(Attr::Class("hide-form".to_string()))
                .add_attr(Attr::Id("dialog".to_string()))
                .adopt(
                    Html::Div()
                        .add_attr(Attr::Id("dialog-event-name-container".to_string()))
                        .adopt(
                            Html::Label()
                                .add_attr(Attr::Custom("for".to_string(), "event-name".to_string()))
                                .set_text("Response Event Name: "),
                        )
                        .adopt(
                            Html::Input()
                                .add_attr(Attr::Type("text".to_string()))
                                .add_attr(Attr::Id("dialog-event-name".to_string()))
                                .add_attr(Attr::Custom(
                                    "name".to_string(),
                                    "event-name".to_string(),
                                )),
                        ),
                )
                .adopt(
                    Html::Div()
                        .add_attr(Attr::Id("cr-event-args-container".to_string()))
                        .adopt(
                            Html::Label()
                                .add_attr(Attr::Custom("for".to_string(), "event-args".to_string()))
                                .set_text("Args: "),
                        )
                        .adopt(
                            Html::TextArea()
                                .add_attr(Attr::Type("text".to_string()))
                                .add_attr(Attr::Id("event-args".to_string()))
                                .add_attr(Attr::Custom(
                                    "name".to_string(),
                                    "event-args".to_string(),
                                )),
                        ),
                )
                .adopt(event_form_btn()),
        )
        .adopt(
            Html::Div()
                .add_attr(Attr::Id("event-args-example".to_string()))
                .adopt(
                    Html::Label()
                        .add_attr(Attr::Custom(
                            "for".to_string(),
                            "event-args-example".to_string(),
                        ))
                        .set_text("Example Args: "),
                )
                .adopt(
                    Html::TextArea()
                        .set_text(str)
                        .add_attr(Attr::Custom("disabled".to_string(), "true".to_string()))
                        .add_attr(Attr::Id("event-args-example".to_string())),
                ),
        )
}

fn event_form_btn() -> Html {
    Html::Button()
        .set_text("Send")
        .add_attr(Attr::Id("event-form-btn".to_string()))
        .add_attr(Attr::Type("button".to_string()))
        .add_attr(Attr::Click(Event::new(
            "event-form-submit".to_string(),
            None,
        )))
}

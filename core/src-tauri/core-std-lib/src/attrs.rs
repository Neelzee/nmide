pub enum Attr {
    /// ```html
    /// <div id="foobar"/>
    /// ```
    Id(String),
    /// ```html
    /// <div class="foobar"/>
    /// ```
    Class(String),
    Style(String),
    /// only valid for `input`
    /// ```html
    /// <input type="checkbox"/>
    /// ```
    Type(String),
    /// only valid for `input`
    /// ```html
    /// <input type="checkbox" checked="true"/>
    /// ```
    Checked(bool),
    /// only valid for `input`
    /// given this dom
    /// ```html
    /// <div id="foobar"/>
    /// ```
    /// the `onclick`-attributes has this effect on an html-element:
    /// ```javascript
    /// const el = document.getelementbyid("foobar");
    /// el.addeventlistener("click", () => {
    ///   window.emit("msg", msg);
    /// });
    /// ```
    /// where `msg` is the value [`tmsg`] in `onclick`
    ///
    /// [`tmsg`]: ../msg/mod.rs
    OnClick(),
    /// only valid for `input`
    /// given this dom
    /// ```html
    /// <div id="foobar"/>
    /// ```
    /// the `oninput`-attributes has this effect on an html-element:
    /// ```javascript
    /// const el = document.getelementbyid("foobar");
    /// el.addeventlistener("input", () => {
    ///   window.emit("msg", msg);
    /// });
    /// ```
    /// where `msg` is the value [`tmsg`] in `oninput`
    ///
    /// [`tmsg`]: ../msg/mod.rs
    OnInput(),
    /// only valid for `input`
    /// given this dom
    /// ```html
    /// <div id="foobar"/>
    /// ```
    /// the `onclick`-attributes has this effect on an html-element:
    /// ```javascript
    /// const el = document.getelementbyid("foobar");
    /// el.addeventlistener("input", () => {
    ///   window.emit("msg", { msg: [msgname, el.value] });
    /// });
    /// ```
    /// where `msgname` is the value in `emitinput`, and the name of the [`msg`] being sent.
    ///
    /// [`msg`]: ../msg/mod.rs
    EmitInput(String),
    /// only valid for `img`, `video`, `audio`, and `script`
    /// ```html
    /// <img src="foobar"/>
    /// ```
    Src(String),
}

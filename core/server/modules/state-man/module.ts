import { click, cls, HtmlBuilder, id, isTBool, isTFloat, isTHtml, isTInt, isTList, isTObj, isTStr, mkPrimEvent, UiBuilder, type Core, type CoreModification, type Event, type Module, type Value } from "@nmide/js-utils";

const stateMan = new HtmlBuilder()
  .attrs(id("state-man"))
  .kids(
    new HtmlBuilder()
      .kind("button")
      .attrs(
        cls("state-man-bbtn"),
        click(mkPrimEvent("toggle-state-man"))
      ),
    new HtmlBuilder()
      .attrs(
        id("state-man-container"),
        cls("state-man-hide"),
      )
  );

const stateField = (field: string, val: Value): HtmlBuilder => {
  const valueKind = (field: string, val: Value): HtmlBuilder => {
    let builder = new HtmlBuilder()

    if (isTBool(val)) {
      builder = builder
        .kind("select")
        .attrs(cls("state-bool"), id(field))
        .kids(
          new HtmlBuilder().kind("option").text("true").attrs(val.bool ? { custom: ["selected", ""] } : undefined),
          new HtmlBuilder().kind("option").text("false").attrs((!val.bool) ? { custom: ["selected", ""] } : undefined),
        );
    } else if (isTInt(val)) {
      builder = builder
        .kind("input")
        .attrs(
          cls("state-int"),
          id(field),
          { type: "number" },
          { custom: ["name", "value"] },
        )
        .text(`${val.int}`);
    } else if (isTFloat(val)) {
      builder = builder
        .kind("input")
        .attrs(
          cls("state-float"),
          id(field),
          { type: "number" },
          { custom: ["name", "value"] },
        )
        .text(`${val.float}`);
    } else if (isTStr(val)) {
      builder = builder
        .attrs(
          cls("state-str"),
          id(field),
          { custom: ["name", "value"] },
        )
        .text(val.str);
    } else if (isTList(val)) {
      builder = builder
        .attrs(
          cls("state-list state-collapsed"),
          id(field),
          { custom: ["name", "value"] },
        )
        .kids(
          ...val.list.map((el, i) => valueKind(`${field}-${i}`, el))
        )
    } else if (isTObj(val)) {
      builder = builder
        .attrs(
          cls("state-obj state-collapsed"),
          id(field),
          { custom: ["name", "value"] },
        )
        .kids(
          ...Object.keys(val.obj).map((k, i) => valueKind(`${field}-${i}`, val.obj[k]!!))
        );
    } else if (isTHtml(val)) {
      builder = builder
        .attrs(
          cls("state-html state-collapsed"),
          id(field),
          { custom: ["name", "value"] },
        )
        .kids(val.html);
    } else if (val === "null") {
      builder = builder
        .kind("input")
        .attrs(
          cls("state-html"),
          id(field),
          { custom: ["name", "value"] },
          { custom: ["readonly", "true"] },
        ).text("null");
    } else {
      window.__nmideConfig__.log.error(`Could not find value-kind: ${JSON.stringify(val)}`);
    }

    return builder;
  }

  return new HtmlBuilder()
    .kind("form")
    .attrs(cls("state-field"))
    .kids(
      new HtmlBuilder().kind("label").attrs({ custom: ["for", "value"] }),
      valueKind(field, val),
      new HtmlBuilder().kind("button").attrs(click(mkPrimEvent("state-man-update")))
    );
}
const module: Module = {
  name: "state-man",
  init: async function(core: Core) {
    await core.registerHandler("state-man", "state-man-update");
    const state = await core.state();
    const keys = Object.keys(state);
    await core.sendModification(new UiBuilder()
      .add(
        new HtmlBuilder()
          .attrs(id("state-man"))
          .kids(
            new HtmlBuilder()
              .kind("button")
              .attrs(
                cls("state-man-bbtn"),
                click(mkPrimEvent("toggle-state-man"))
              ),
            new HtmlBuilder()
              .attrs(
                id("state-man-container"),
                cls("state-man-hide"),
              )
              .kids(
                ...keys.map(k => stateField(k, state[k]!!))
              )
          )
      )
      .build());
  },
  handler: async function(event: Event, core: Core) {
    throw new Error("Function not implemented.");
  }
}

export default module;

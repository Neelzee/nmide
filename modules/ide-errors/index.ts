import {
  CoreModification,
  Core,
  Event,
  emptyCm,
  Value,
  UiBuilder,
  installModule, HtmlBuilder,
  isPrimitiveEvent
} from "@nmide/js-utils";


installModule(
  {
    name: "ide-errors",
    init: async (core: Core): Promise<void> => {
      await core.registerHandler("ide-errors", "fsa-errors");
    },
    handler: async (event: Event, core: Core): Promise<void> => {
      if (!isPrimitiveEvent(event)) return;
      const { args } = event.event;
      if (args === null) return;
      const kp: [string, Value][] = Object.keys(args).map(k => [k, args[k]]);
      await core.sendModification(new UiBuilder()
        .add(
          new HtmlBuilder()
            .kind("li")
            .kids(
              ...kp.map(
                ([f, v]) => new HtmlBuilder()
                  .kind("span")
                  .kids(
                    new HtmlBuilder().kind("label").text(f),
                    new HtmlBuilder().kind("p").text(JSON.stringify(v))
                  )
              )
            )
          ,
          "errors"
        ).build());
    },
  }
);

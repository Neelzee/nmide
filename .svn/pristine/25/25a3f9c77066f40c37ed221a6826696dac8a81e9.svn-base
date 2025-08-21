import {
  CoreModification,
  Core,
  Event,
  emptyCm,
  Value,
  UiBuilder,
  installModule, HtmlBuilder
} from "@nmide/js-utils";


installModule(
  {
    name: "ide-errors",
    init: async (core: Core): Promise<CoreModification> => {
      await core.registerHandler("ide-errors", "fsa-errors");
      return emptyCm();
    },
    handler: async (event: Event, _: Core): Promise<CoreModification> => {
      const { args } = event;
      if (args === null) return emptyCm();
      const kp: [string, Value][] = Object.keys(args).map(k => [k, args[k]]);
      return new UiBuilder()
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
        ).build();
    },
  }
);
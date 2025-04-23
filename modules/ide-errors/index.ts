import {
  CoreModification,
  Core,
  Event,
  emptyCm,
  Value,
  Html,
  UiBuilder,
  installModule
} from "@nmide/js-utils";


installModule(
  {
    name: "ide-errors",
    init: async (core: Core): CoreModification => {
      await core.registerHandler("ide-errors", "fsa-error", null);
      return emptyCm();
    },
    handler: async (event: Event, core: Core): CoreModification => {
      const { args } = event;
      const kp: [string, Value][] = Object.keys(args).map(k => [k, args[k]]);
      const elm: Html = {
        li: {
          kids: kp.map(([f, v]) => {
            return {
              span: {
                kids: [
                  { label: { kids: [], attrs: [], text: f } },
                  { p: { kids: [], attrs: [], text: JSON.stringify(v) } },
                ],
                attrs: [],
                text: null,
              }
            };
          }),
          attrs: [],
          text: "",
        }
      };
      return new UiBuilder()
        .add(elm, "errors", null)
        .build();
    },
  }
);
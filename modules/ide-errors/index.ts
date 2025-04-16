import {
  CoreModification,
  Core,
  Event,
  emptyCm,
  Value,
  HtmlBuilder,
  Html
} from "@nmide/js-utils";

const module_name = "ide-errors";
window.__nmideConfig__.modules.set(
  module_name,
  {
    init: async (core: Core): CoreModification => {
      await core.registerHandler(module_name, "fsa-error", null);
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
      return new HtmlBuilder()
        .add(elm, "errors", null)
        .build();
    },
  }
)
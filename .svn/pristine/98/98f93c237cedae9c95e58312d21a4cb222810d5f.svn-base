import { Core, CoreManager, CoreModification } from "@nmide/js-utils/lib/Core";
import HtmlBuilder from "@nmide/js-utils/lib/HtmlBuilder";
import { isTInt, tInt, TValue } from "@nmide/js-utils";

const module = "CounterPlugin";
window.plugins.set(
  module,
  {
    init: async (core: Core): Promise<CoreModification> => {
      const cb = new CoreManager(core)
        .addEvent({ event: "counter-event", module: module })
        .addEventHandler(
          "counter-event",
          {
            module,
            handler: async (c: Core, ..._: TValue[]) => {
              return new CoreManager(c)
                .modifyField("count", t => {
                  if (isTInt(t)) {
                    t.int += 1;
                    return t;
                  } else {
                    return tInt(-1);
                  }
                }).build();
            }
          })
        .addField("count", tInt(0));
      return cb
        .addUI(
          new HtmlBuilder()
            .kind("div")
            .kids(new HtmlBuilder()
              .kind("button")
              .attrs({ onClick: { msg: ["counter-event", tInt(1)] } })
              .text(`Count: ${cb.findField("count")}`)
            ).build(),
          _ => true
        )
        .build();
    }
  }
)

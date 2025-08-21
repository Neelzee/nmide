import { Core, CoreManager, CoreModification } from "@nmide/js-utils/lib/Core";
import HtmlBuilder from "@nmide/js-utils/lib/HtmlBuilder";
import { tInt } from "@nmide/js-utils";

const module = "CounterPlugin";
window.plugins.set(
  module,
  {
    init: async (core: Core): Promise<CoreModification> => {
      const cb = new CoreManager(core);
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

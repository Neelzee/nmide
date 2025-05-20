import { emptyCm, installModule, isPrimAnd, StateBuilder, type Core, type CoreModification, type Event } from "@nmide/js-utils";
import { run } from "./unPureCode";

installModule({
  name: "ide_explorer_expanded",
  init: async function(core: Core): Promise<void> {
    await core.registerHandler("ide_explorer_expanded", "open-project-post");
  },
  handler: async function(event: Event, core: Core): Promise<void> {
    if (isPrimAnd(event, "open-project-post")) {
      let val = (await core.state())["ide_explorer_expanded_init"];
      if (val === undefined) {
        run();
        await core.sendModification(new StateBuilder().add("ide_explorer_expanded_init", null).build());
      }
    }
  }
})

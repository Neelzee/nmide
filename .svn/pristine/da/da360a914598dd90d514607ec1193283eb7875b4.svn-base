import { emptyCm, installModule, isPrimAnd, StateBuilder, type Core, type CoreModification, type Event } from "@nmide/js-utils";
import { run } from "./unPureCode";

installModule({
  name: "ide_explorer_expanded",
  init: async function (core: Core): Promise<CoreModification> {
    await core.registerHandler("ide_explorer_expanded", "open-project-post");
    return emptyCm();
  },
  handler: async function (event: Event, core: Core): Promise<CoreModification> {
    if (isPrimAnd(event, "open-project-post")) {
      let val = (await core.state())["ide_explorer_expanded_init"];
      if (val === undefined) {
        run();
        return new StateBuilder().add("ide_explorer_expanded_init", null).build();
      }
    }
    return emptyCm();
  }
})
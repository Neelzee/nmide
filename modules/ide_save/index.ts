import {
  installModule,
  isPrimAnd,
  mkPrimEvent,
  tStr,
  type Core,
  type Event
} from "@nmide/js-utils";

const name = "ide_save";
const event_name = "ide-save";

installModule({
  name,
  init: async (core: Core): Promise<void> => {
    await core.registerHandler(name, event_name);
  },
  handler: async (event: Event, core: Core): Promise<void> => {
    if (isPrimAnd(event, event_name)) {
      const txt = document.getElementById("editor");
      if (txt === null) return;
      const file_path = tStr(txt.getAttribute("class")?.split(" ").filter(e => e !== "code-editor")[0]?.replaceAll("_", "/") || "");
      console.log("fp: ", file_path);
      if (txt instanceof HTMLTextAreaElement) {
        const content = tStr(txt.value);
        await core.eventThrower(
          mkPrimEvent("fsa-write", { obj: { file_path, content } })
        );
      }
    }
  }
})

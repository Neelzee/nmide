import module from "./module";
import { debug_module } from "@nmide/js-debug-module";
import { renderer } from "../../core/app/lib/tsRenderer";
import type { Event } from "@nmide/js-utils";

const core = {
  eventThrower: async (evt: Event) => {
    window.debug_module.handler(evt)
      .catch(console.error);
  }
}

document.addEventListener("DOMContentLoaded", () => {
  debug_module(module, core, renderer(core.eventThrower));
})
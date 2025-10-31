import { PrimitiveEvent, Event } from "@nmide/js-utils";
import module from "./module";
import { debug_module } from "@nmide/js-debug-module";
import { renderer } from "../../core/app/lib/tsRenderer";

const core = {
  eventThrower: async (evt: Event) => {
    window.debug_module.handler(evt)
      .catch(console.error);
  }
}

document.addEventListener("DOMContentLoaded", () => {
  debug_module(module, core, renderer(core.eventThrower));
})
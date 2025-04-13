import { empty_core_modification, Core, Event } from "lib_gleam";

const module = "CounterPlugin";
// @ts-ignore
window.__nmideConfig__.modules.set(
  module,
  {
    name: module,
    init: async (core: Core) => {
      core.registrate_handler(module, "counter", null)
        .catch(err => console.error("error from module: ", err));
      return empty_core_modification();
    },
    handler: async (event: Event, __: Core) => {
      console.log("Module got event: ", event);
      return empty_core_modification();
    },
  }
);

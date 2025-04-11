import { empty_core_modification, build_module, Core, CoreModification, Event } from "core_modification";

const module = "CounterPlugin";
// @ts-ignore
window.modules.set(
  module,
  {
    name: module,
    init: async (_: Core) => empty_core_modification(),
    handler: async (_: Event, __: Core) => empty_core_modification(),
  }
);

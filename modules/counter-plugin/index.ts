import { empty_core_modification, build_module, Core, CoreModification, Event } from "core_modification";

const module = "CounterPlugin";
console.log(empty_core_modification());

const init = (core: Core): Promise<CoreModification> => {
  throw new Error();
};

const handler = (event: Event, core: Core): Promise<CoreModification> => {
  throw new Error();
};

const mod = build_module(module, init, handler);

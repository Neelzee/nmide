import { Core } from "./Core";
import { Event } from "./Event";
import { CoreModification } from "./CoreModification";

export interface Module {
  name: string;
  init: (core: Core) => Promise<CoreModification>;
  handler: (event: Event, core: Core) => Promise<CoreModification>;
}

/**
 * All results from modules are by default, unknown.
 */
export interface ModuleUnknown {
  name: string;
  init: (core: Core) => Promise<unknown>;
  handler: (event: Event, core: Core) => Promise<unknown>;
}

export const installModule = (module: Module): void => {
  document.addEventListener("nmide://ModulesInstalled", () => {
    window.__nmideConfig__.modules.set(module.name, module);
  });
}
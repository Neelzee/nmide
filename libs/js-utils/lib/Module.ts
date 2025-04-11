import { Core, CoreModification, Event } from "./Core";

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

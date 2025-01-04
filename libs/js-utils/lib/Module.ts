import { Core } from "./Core";


export interface Module {
  init: (core: Core) => Promise<Core>;
};

/**
 * All results from modules are by default, unknown.
 */
export interface ModuleUnknown {
  init: (core: Core) => Promise<unknown>;
};

import "@nmide/js-utils";
import {
  CoreModification,
  Module,
  Core
} from "core_modification";

// TODO: Add docs
export const init = (
  modules: Module[],
  core: Core,
): Promise<CoreModification[]> =>
  // TODO: Add validation
  Promise.all(modules.map(m => m.init(core)));
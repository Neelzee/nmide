import type { Core } from "./Core";
import { formatValidationErrors } from "io-ts-reporters";
import { DModule } from "@nmide/js-decoder-lib";
import * as E from "fp-ts/Either";
import moduleWrapper from "@nmide/js-module-lib/lib/module_handler";
import { type Event } from "./Event";

export interface Module {
  name: string;
  init: (core: Core) => Promise<void>;
  handler: (event: Event, core: Core) => Promise<void>;
}

/**
 * All results from modules are by default, unknown.
 */
export interface ModuleUnknown {
  name: string;
  init: Function;
  handler: Function;
}

export const installModule = (module: Module): void => {
  const mod = E.mapLeft(formatValidationErrors)(DModule.decode(module));
  if (E.isLeft(mod)) {
    window.__nmideConfig__
      .log
      .error(`Error on module installation: ${JSON.stringify(mod.left)}, input module: ${JSON.stringify(module)}`);
    return;
  }
  const m = moduleWrapper(mod.right);
  document.addEventListener("nmide://ModulesInstalled", () => {
    window.__nmideConfig__.modules.set(m.name, m);
  });
}

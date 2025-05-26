import type { Core } from "./Core";
import { formatValidationErrors } from "io-ts-reporters";
import { DModule } from "@nmide/js-decoder-lib";
import * as E from "fp-ts/Either";
import moduleWrapper from "@nmide/js-module-lib/lib/module_handler";
import { type Event } from "./Event";
import {
  NMIDE_INITIALIZED,
  NMIDE_RT_MODULE_INSTALLED_EVENT,
  NMIDE_RT_MODULE_PUSHED_EVENT,
} from "@nmide/js-core-std-lib";

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
  document.addEventListener(
    NMIDE_INITIALIZED,
    () => {
      window.__nmideConfig__.modules.set(m.name, m);
    },
    { once: true }
  );
  document.addEventListener(
    NMIDE_RT_MODULE_INSTALLED_EVENT,
    () => {
      if (
        window.__nmideConfig__.modules.get(m.name) === undefined &&
        window.__nmideConfig__.rt_modules.get(m.name) === undefined
      ) {
        document.dispatchEvent(
          new CustomEvent(
            NMIDE_RT_MODULE_PUSHED_EVENT,
            { detail: m }
          )
        )
      }
    },
    { once: true }
  );
}

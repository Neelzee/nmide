import { Core as GleamCore } from "../../build/dev/javascript/nmide_gleam_utils/gleam/utils/core.mjs";
import type { Core } from "@nmide/js-utils";

export function from_js_core(core: Core) {
  return new GleamCore(core.state, core.ui, core.eventThrower, core.registerHandler, core.sendModification);
}
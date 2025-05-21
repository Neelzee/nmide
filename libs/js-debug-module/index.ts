import { defaultConfig, parseStateInstr } from "@nmide/js-core-std-lib";
import { DebugCore, type Core, type Module, type Event, type CoreModification, emptyCm, mkPrimEvent, type Value, type State, type NmideConfig } from "@nmide/js-utils";

export const debug_module = (
  m: Partial<Module>,
  core?: Partial<Core>,
  initial_state?: State,
  render?: NmideConfig["render"],
  config?: Partial<NmideConfig>,
) => {
  window.__nmideConfig__ = { ...defaultConfig, ...config };
  window.debug_state = initial_state || {};
  const c = core === undefined
    ? DebugCore()
    : { ...DebugCore(), state: () => new Promise<State>(r => r(window.debug_state)), ...core };
  window.debug_module = {
    init: async () => {
      m.init?.(c);
    },
    handler: async (event?: Event) => {
      m?.handler?.(
        event === undefined
          ? mkPrimEvent("DebugEvent", null)
          : event,
        c
      );
    }
  }
}

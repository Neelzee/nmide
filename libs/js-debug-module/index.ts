import { defaultConfig, parseStateInstr, type NmideConfig } from "@nmide/js-core-std-lib";
import { DebugCore, type Core, type Module, type Event, type CoreModification, emptyCm, mkPrimEvent, type Value, type State } from "@nmide/js-utils";

declare global {
  interface Window {
    debug_module: {
      init: () => Promise<void>,
      handler: (event?: Event) => Promise<void>
    };
    __nmideConfig__: NmideConfig,
    debug_state: State,
  }
}

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
      const promise = m?.init === undefined ? new Promise<CoreModification>(r => r(emptyCm())) : m.init(c);
      const result = await promise;
      if (render !== undefined) {
        render(result.ui);
      }
      window.debug_state = parseStateInstr(result.state)(window.debug_state);
    },
    handler: async (event?: Event) => {
      const promise = m?.handler === undefined
        ? new Promise<CoreModification>(r => r(emptyCm()))
        : m.handler(
          event === undefined
            ? mkPrimEvent("DebugEvent", null)
            : event,
          c
        );
      const result = await promise;
      if (render !== undefined) {
        render(result.ui);
      }
      window.debug_state = parseStateInstr(result.state)(window.debug_state);
    }
  }
}
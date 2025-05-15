import { defaultConfig, parseStateInstr, type NmideConfig } from "@nmide/js-core-std-lib";
import { DebugCore, type Core, type Module, type Event, type CoreModification, emptyCm, mkPrimEvent, type Value, type State } from "@nmide/js-utils";

declare global {
  interface Window {
    debug_module: {
      init: () => Promise<void>,
      handler: (event?: Event) => Promise<void>
    };
    __nmideConfig__: NmideConfig,
    state: Record<string, Value | undefined>,
  }
}

export const debug_module = (
  m: Partial<Module>,
  core?: Partial<Core>,
  render?: NmideConfig["render"],
  config?: Partial<NmideConfig>,
) => {
  window.__nmideConfig__ = { ...defaultConfig, ...config };
  window.state = window.state === undefined ? {} : window.state;
  const c = core === undefined
    ? DebugCore()
    : { ...DebugCore(), state: () => new Promise<State>(r => r(window.state)), ...core };
  window.debug_module = {
    init: async () => {
      const promise = m?.init === undefined ? new Promise<CoreModification>(r => r(emptyCm())) : m.init(c);
      const result = await promise;
      if (render !== undefined) {
        render(result.ui);
      }
      window.state = parseStateInstr(result.state)(window.state);
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
      window.state = parseStateInstr(result.state)(window.state);
    }
  }
}
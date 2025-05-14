import { defaultConfig, type NmideConfig } from "@nmide/js-core-std-lib";
import { DebugCore, type Core, type Module, type Event, type CoreModification, emptyCm, mkPrimEvent } from "@nmide/js-utils";

declare global {
  interface Window {
    debug_module: {
      init: () => Promise<void>,
      handler: (event?: Event) => Promise<void>
    };
    __nmideConfig__: NmideConfig,
  }
}

export const debug_module = (
  m: Partial<Module>,
  core?: Partial<Core>,
  render?: NmideConfig["render"],
  config?: Partial<NmideConfig>,
) => {
  window.__nmideConfig__ = { ...defaultConfig, ...config };
  const c = core === undefined ? DebugCore() : { ...DebugCore(), ...core };
  window.debug_module = {
    init: async () => {
      const promise = m?.init === undefined ? new Promise<CoreModification>(r => r(emptyCm())) : m.init(c);
      const result = await promise;
      if (render !== undefined) {
        render(result.ui);
      }
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
    }
  }
}
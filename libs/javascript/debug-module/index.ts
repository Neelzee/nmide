import { defaultConfig } from "@nmide/js-core-std-lib";
import {
  DebugCore,
  type Core,
  type Module,
  type Event,
  type CoreModification,
  mkPrimEvent,
  type State,
  type NmideConfig
} from "@nmide/js-utils";

/**
 * Core imitation for debugging JavaScript modules in the browser.
 * For the UI to work, a renderer needs to be passed.
 */
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
    : {
      ...DebugCore(),
      state: () => new Promise<State>(r => r(window.debug_state)),
      sendModification: async (modification: CoreModification) => {
        console.log("Core.sendModification", modification);
        const ui = modification.ui;
        render?.(ui);
      },
      ...core
    };
  window.debug_module = {
    init: async () => {
      await m.init?.(c);
    },
    handler: async (event?: Event) => {
      await m?.handler?.(
        event === undefined
          ? mkPrimEvent("DebugEvent", "null")
          : event,
        c
      );
    }
  }
}

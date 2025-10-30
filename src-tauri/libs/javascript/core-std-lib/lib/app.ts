import type { AppConfig, NmideConfig } from "@nmide/js-utils";

export interface App {
  initialize: (config: Partial<AppConfig>) => Promise<void>;
  run: () => Promise<void>;
}

export const defaultConfig: NmideConfig = {
  /**
   * Logging
   *
   * Makes it possible for different applications to implement and inject their
   * own logging solution.
   */
  log: {
    error: console.error,
    debug: console.debug,
    info: console.log
  },
  moduleInstallers: [],
  root: document.body,
  runtimes: { handlers: [], initializers: [] },
  /**
   * Gets UI instructions, and _writes_ them to the DOM (NmideConfig.root).
   */
  render: _ => {
    throw Error("Missing renderer");
  },
  handlerRegistration: _ => {
    throw Error("Missing handlerRegistration");
  },
  eventThrower: _ => {
    throw Error("Missing eventThrower");
  },
  moduleCount: 0,
  modules: new Map(),
  rt_modules: new Map(),
  installed: false,
  handlerRegister: { event: new Map() },
  events: []
};

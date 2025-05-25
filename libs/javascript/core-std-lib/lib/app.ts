import type { AppConfig, NmideConfig } from "@nmide/js-utils";

export interface App {
  initialize: (config: Partial<AppConfig>) => Promise<void>;
  run: () => Promise<void>;
}

export const defaultConfig: NmideConfig = {
  log: {
    error: console.error,
    debug: console.debug,
    info: console.log
  },
  moduleInstallers: [],
  root: document.body,
  runtimes: { handlers: [], initializers: [] },
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

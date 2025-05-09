import {
  Instruction,
  Module,
  Event,
  Html,
  Attr,
  CoreModification
} from "@nmide/js-utils";


export interface App {
  initialize: (config: Partial<AppConfig>) => void;
  installModules: () => void;
  run: () => void;
}

export interface AppConfig {
  root: HTMLElement,
  log: {
    error: (...args: unknown[]) => void,
    debug: (...args: unknown[]) => void,
    info: (...args: unknown[]) => void,
  },
  moduleInstallers: (() => Promise<void>)[],
  runtimes: {
    initializers: (() => Promise<CoreModification[]>)[]
    handlers: ((event: Event) => Promise<CoreModification[]>)[]
  },
  render: (ui: [Instruction<Html>, Instruction<string>, Instruction<Attr>]) => Promise<void>,
  eventThrower: (event: Event) => Promise<void>,
  handlerRegistration: (
    module: string,
    event_name?: string,
  ) => Promise<void>
}

export type HandlerRegister = {
  event: Map<string, string[]>,
  module: Map<string, string[]>,
}

export interface NmideConfig extends AppConfig {
  moduleCount: number,
  modules: Map<string, Module>,
  handlerRegister: HandlerRegister
  installed: boolean,
}

declare global {
  interface Window {
    __nmideConfig__: NmideConfig;
  }
}

export const defaultConfig = (
  render: NmideConfig["render"],
  handlerRegistration: NmideConfig["handlerRegistration"],
  eventThrower: NmideConfig["eventThrower"]
): NmideConfig => {
  return {
    log: {
      error: console.error,
      debug: console.debug,
      info: console.log
    },
    moduleInstallers: [],
    root: document.body,
    runtimes: { handlers: [], initializers: [] },
    render,
    handlerRegistration,
    eventThrower,
    moduleCount: 0,
    modules: new Map(),
    installed: false,
    handlerRegister: { event: new Map(), module: new Map() }
  };
};

import { Html, TUIInstruction } from "@nmide/js-utils/lib/Html";
import { Instruction, Module, TEvent } from "@nmide/js-utils";
import { tsRenderer } from "./lib/tsRenderer.ts";
import { handlerRegistration } from "./lib/handlerRegistration.ts";
import { eventThrower } from "./lib/eventThrower.ts";
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
    handlers: ((event: TEvent) => Promise<CoreModification[]>)[]
  },
  render: (op: [Instruction<Html>, Instruction<string>, Instruction<Attr>]) => Promise<void>,
  eventThrower: (event: TEvent) => Promise<void>,
  handlerRegistration: (
    module: string,
    event_name: string | null,
    module_name: string | null
  ) => Promise<void>
}

export type HandlerRegister = {
  event: Map<string, string[]>,
  module: Map<string, string[]>,
}

interface NmideConfig extends AppConfig {
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

export const defaultConfig: NmideConfig = {
  log: {
    error: console.error,
    debug: console.debug,
    info: console.log
  },
  moduleInstallers: [],
  root: document.body,
  runtimes: { handlers: [], initializers: [] },
  render: tsRenderer,
  handlerRegistration,
  eventThrower,
  moduleCount: 0,
  modules: new Map(),
  installed: false,
  handlerRegister: { event: new Map(), module: new Map() }
}

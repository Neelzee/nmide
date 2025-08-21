import { type CoreModification } from "./lib/CoreModification";
import type { Html } from "./lib/Html";
import type { Instruction } from "./lib/Instruction";
import { type State } from "./lib/State";
import type { Event } from "./lib/Event";
import { type Module } from "./lib/Module";
import { type Attr } from "./lib/Attr";

export interface AppConfig {
  root: HTMLElement,
  log: {
    error: (...args: unknown[]) => void,
    debug: (...args: unknown[]) => void,
    info: (...args: unknown[]) => void,
  },
  moduleInstallers: (() => Promise<void>)[],
  runtimes: {
    initializers: (() => Promise<void>)[]
    handlers: ((event: Event) => Promise<void>)[]
  },
  render: (ui: [Instruction<Html>, Instruction<string>, Instruction<Attr>]) => Promise<void>,
  eventThrower: (event: Event) => Promise<void>,
  handlerRegistration: (
    module: string,
    event_name?: string,
  ) => Promise<void>,
  events: Event[],
}

export type HandlerRegister = {
  event: Map<string, string[]>,
}

export interface NmideConfig extends AppConfig {
  moduleCount: number,
  modules: Map<string, Module>,
  rt_modules: Map<string, Module>,
  handlerRegister: HandlerRegister
  installed: boolean,
}

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

export * from "./lib/HtmlBuilder";
export * from "./lib/UiBuilder";
export * from "./lib/Html";
export * from "./lib/Eq";
export * from "./lib/Module";
export * from "./lib/Types";
export * from "./lib/Utils";
export * from "./lib/Attr";
export * from "./lib/Core";
export * from "./lib/State";
export * from "./lib/Event";
export * from "./lib/Instruction";
export * from "./lib/CoreModification";
export * from "./lib/StateBuilder";
export * from "./lib/AttrUtils.ts";
export * from "./lib/EventUtils.ts";
export * from "./lib/DevCore.ts";
export * from "./lib/HtmlUtils.ts";

export const emptyCm = (): CoreModification => {
  return {
    state: "noOp",
    ui: ["noOp", "noOp", "noOp"]
  };
}

export const emptyState = (): State => {
  return {};
}

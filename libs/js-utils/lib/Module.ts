import { THtml } from "./THtml";
import { TValue } from "./TMap";
import TreeManager, { createTreeManager } from "./tree";
import { tObj } from "./Types";


export interface Module {
  init: (core: Core) => Promise<Core>;
};

/**
 * All results from modules are by default, unknown.
 */
export interface ModuleUnknown {
  init: (core: Core) => unknown;
};

export type Event = {
  // Event name
  event: string,
  // Module id
  module: string,
};

export type EventHandler = (c: Core, ...args: TValue[]) => Promise<Core>;

export type Core = {
  /**
   * UI Hierarchy
   */
  ui: TreeManager<THtml>;
  /**
   * State of the application
   */
  state: TreeManager<TValue>;
  /**
   * List of events
   */
  events: Event[],
  /**
   * List of event handlers
   *
   * event name -> module name, event handler
   */
  eventHandlers: Map<string, [string, EventHandler][]>
  readonly eventThrower: (e: Event) => void,
};

export const defaultCore: Core = {
  ui: createTreeManager({
    kind: "div",
    kids: [],
    attrs: [{ id: "root" }],
    text: null
  }),
  state: createTreeManager(tObj([["root", []]])),
  events: [
    { event: "UI-Update", module: "Core" },
    { event: "UI-Collision", module: "Core" },
    { event: "State-Update", module: "Core" },
    { event: "State-Collision", module: "Core" },
    { event: "Post-INIT", module: "Core" },
    { event: "Module-Exception", module: "Core" },
  ],
  eventHandlers: new Map(),
  eventThrower: _ => { throw new Error("Unimplemented") },
};

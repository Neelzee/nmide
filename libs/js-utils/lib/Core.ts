import { THtml } from "./THtml";
import { TValue } from "./TMap";
import { Ins } from "./instruction";
import { Node } from "./tree";

export type EventHandler = (c: Core, ...args: TValue[]) => Promise<unknown>;

export type Event = {
  // Event name
  event: string,
  // Module id
  module: string,
  args?: TValue[],
}

export type Core = {
  /**
   * UI Hierarchy
   */
  ui: THtml;
  /**
   * State of the application
   */
  state: TValue;
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
  // TODO: Is this the way to go?
  readonly eventThrower: (e: Event) => void,
}

// TODO: How do I expose a JSON-safe way to manage/handle the Core?:xz
export type CoreInstructor<T> = {
  getNode: (f: ((node: Node<T>) => Promise<boolean>), c: Core) => string | undefined,
  addNode: (n: Node<T>) => Ins<T>,
  removeNode: (id: string) => Ins<T>,
  modifyNode: (id: string, g: ((node: Node<T>) => Promise<Node<T>>)) => Ins<T>,
};

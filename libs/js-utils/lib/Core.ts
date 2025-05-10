// TODO: Add docs

import { Html } from "./Html";
import { Event } from "./Event";
import { State } from "./State";

export type Core = {
  readonly ui: () => Promise<Html>;
  /**
   * State of the application
   */
  readonly state: () => Promise<State>;
  /**
   * List of events
   */
  readonly eventThrower: (evt: Event) => Promise<void>;
  readonly registerHandler: (name: string, event?: string, module?: string) => Promise<void>;
};
// TODO: Add docs

import type { Html } from "./Html";
import type { Event } from "./Event";
import type { State } from "./State";
import type { CoreModification } from "./CoreModification";

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
  readonly registerHandler: (name: string, event: string) => Promise<void>;
  readonly sendModification: (modification: CoreModification) => Promise<void>;
};

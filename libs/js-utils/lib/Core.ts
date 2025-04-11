// TODO: Add docs

import { TValue } from "./TMap";
import { Eq, fromEquals } from "fp-ts/Eq";
import { Eq as SEq } from "fp-ts/string";
import { Html } from "./Html";

export type Event = {
  // Event name
  event: string,
  // Module id
  module: string,
  // Optional Arguments
  args?: TValue,
};

export const EventEq: Eq<Event> = fromEquals(
  (
    { event: xe, module: xm },
    { event: ye, module: ym }
  ) => SEq.equals(xe, ye) && SEq.equals(xm, ym)
);

export type Core = {
  readonly ui: () => Promise<Html>;
  /**
   * State of the application
   */
  readonly state: () => Promise<object>;
  /**
   * List of events
   */
  readonly eventThrower: (evt: Event) => Promise<void>;
};
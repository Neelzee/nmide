import { Event, EventEq, EventHandler } from "@nmide/js-utils/lib/Core";
import * as A from "fp-ts/Array";
import * as O from "fp-ts/Option";
import * as M from "fp-ts/Map";
import { Eq as SEq } from "fp-ts/string";
import { pipe } from "fp-ts/lib/function";

export const addEvent = (event: Event) =>
  A.elem(EventEq)(event)(window.events)
    ? window.log.info(`Event: ${event.event} from module ${event.module} already exists`)
    : window.events = A.append(event)(window.events);

export const addEventHandler = (event: string, eventHandler: EventHandler) => pipe(
  window.eventHandlers,
  M.lookup(SEq)(event),
  O.match(
    () => {
      window.eventHandlers.set(event, [eventHandler])
    },
    handlers => {
      window.eventHandlers.set(event, A.append(eventHandler)(handlers));
    }
  )
);

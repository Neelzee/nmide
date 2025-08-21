import type { Attr } from "./Attr.ts";
import { type Event } from "./Event.ts";
import { getArgs, getEventName, mkPrimEvent } from "./EventUtils.ts";
import { valueNanCheck } from "./ValueUtils.ts";

export const id = (id: string): { id: string } => {
  return { id };
}

export const cls = (cls: string): { clss: string } => {
  return { clss: cls };
}

export const click = (event: Event): { click: Event } => {
  return { click: event };
}

export const change = (event: Event): { change: Event } => {
  return { change: event };
}

export const idCmp = (a: Attr, id: string): boolean =>
  "id" in a && a.id === id;

export const attrIsEvent = (a: Attr): a is ({ click: Event } | { change: Event }) =>
  "click" in a || "change" in a;

export const getEvent = (a: Attr): Event | undefined => !attrIsEvent(a)
  ? undefined
  : "click" in a
    ? a.click
    : a.change

export const attrsNanCheck = (a: Attr): Attr => {
  if (!attrIsEvent(a)) return a;
  const evt = getEvent(a)!!;
  const args = getArgs(evt);
  if (args !== null) {
    const newArgs = valueNanCheck(args);
    // @ts-expect-error This is valid
    const newEvt = "coreResponse" in evt
      ? { coreResponse: { event: evt.coreResponse.event, args: newArgs } }
      : { event: { event: getEventName(evt), args: newArgs } };
    if ("click" in a) {
      return { click: newEvt };
    } else {
      return { change: newEvt };
    }
  }
  return a;
}

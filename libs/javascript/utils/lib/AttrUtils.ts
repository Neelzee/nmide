import type { Attr } from "./Attr.ts";
import { type Event } from "./Event.ts";

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

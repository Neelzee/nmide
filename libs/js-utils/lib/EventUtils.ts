import { type Event } from "./Event";

export type PrimitiveEvent = Extract<Event, { event: any }>;

export type PostInit = Extract<Event, "postInit">;

export type PreExit = Extract<Event, "preExit">;

export const isPrimitiveEvent = (event: Event): event is PrimitiveEvent =>
  typeof event === "object" && "event" in event;

export const isPostInit = (event: Event): event is PostInit =>
  event === "nmide://post-init";

export const isPreExit = (event: Event): event is PreExit =>
  event === "nmide://post-init";

export const primDec = (event: PrimitiveEvent): PrimitiveEvent["event"] =>
  event.event;

export const isPrimDec = (event: Event): PrimitiveEvent["event"] | undefined =>
  isPrimitiveEvent(event)
    ? primDec(event)
    : undefined

export const isPrimAnd = (event: Event, name: string): event is PrimitiveEvent =>
  isPrimitiveEvent(event) && event.event.event === name;

export const isPrimAndDec = (event: Event, name: string): PrimitiveEvent["event"] | undefined =>
  isPrimAnd(event, name)
    ? primDec(event)
    : undefined

export const getEventName = (event: Event): string =>
  isPostInit(event) || isPreExit(event)
    ? event
    : isPrimitiveEvent(event)
      ? event.event.event
      : "dialogEvent" in event
        ? event.dialogEvent.event
        : "dialogFile" in event
          ? event.dialogFile.event
          : event.coreResponse.event;
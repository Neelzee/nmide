import { type Event } from "./Event";
import { type Value } from "./State";
import { tNull, tValueMaybeOr } from "./Types";

export type PrimitiveEvent = Extract<Event, { event: any }>;

export type PostInit = "nmide://post-init"

export type PreExit = "nmide://pre-exit";

export const isPrimitiveEvent = (event: Event): event is PrimitiveEvent =>
  typeof event === "object" && "event" in event;

export const isPostInit = (event: Event): event is PostInit =>
  event === "nmide://post-init";

export const isPreExit = (event: Event): event is PreExit =>
  event === "nmide://pre-exit";

export const primDec = (event: PrimitiveEvent): PrimitiveEvent["event"] =>
  event.event;

export const isPrimDec = (event: Event): PrimitiveEvent["event"] | undefined =>
  isPrimitiveEvent(event)
    ? primDec(event)
    : undefined

export const isPrimAnd = <T extends string>(
  event: Event,
  name: T
): event is { event: { event: T; args: Value | null } } =>
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


export const mkPrimEvent = (event: string, args?: Value): PrimitiveEvent => {
  return {
    event: {
      event,
      args: tValueMaybeOr(args)(tNull())
    }
  };
}

export const getArgs = (event: Event): Value | null =>
  isPostInit(event) || isPreExit(event)
    ? null
    : isPrimitiveEvent(event)
      ? event.event.args
      : "dialogEvent" in event
        ? null
        : "dialogFile" in event
          ? null
          : event.coreResponse.args;


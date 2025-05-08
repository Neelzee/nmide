import { Event } from "./Event.ts";

export const id = (id: string): { id: string } => {
    return { id };
}

export const cls = (cls: string): { clss: string } => {
    return { clss: cls };
}

export const click = (event: Event): { click: Event } => {
    return { click: event };
}

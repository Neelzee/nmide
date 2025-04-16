import { Event } from "@nmide/js-utils";

import { emit } from "@tauri-apps/api/event";
export const eventThrower = async (event: Event) => {
  emit(
    "nmide://event",
    { event: event.event, module: event.module, args: event.args }
  ).catch(
    err =>
      window.__nmideConfig__.log.error(
        `Event ${event} resulted in error from backend: `, err
      )
  );
}
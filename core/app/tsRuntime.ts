import {
  Core,
  Event,
  CoreModification,
} from "lib_gleam";
import { invoke } from "@tauri-apps/api/core";
import { handlerRegistration } from "./lib/handlerRegistration.ts";
import { eventThrower } from "./lib/eventThrower.ts";

export const tsHandler = async ({ event, module, args }: Event) => {
  const core = {
    state: await invoke<object>("state").catch(err => console.error(err)),
    ui: await invoke<object>("ui").catch(err => console.error(err)),
    registrate_handler: handlerRegistration,
    throw_event: eventThrower,
  };

  const event_modules = window.__nmideConfig__.handlerRegister.event.get(event);
  const module_modules = window.__nmideConfig__.handlerRegister.module.get(module);
  const modules = event_modules === undefined ? [] : event_modules;
  modules.push(...(module_modules === undefined ? [] : module_modules));
  // TODO: Add proper validation/handling
  const modifications: CoreModification[] = await Promise.all(
    modules
      .map(m => window.__nmideConfig__.modules.get(m))
      .filter(m => m !== undefined)
      .map(m => m.handler({ event, module, args }, core))
  );
  return modifications;
}

export const tsInit = async () => {

  const core = {
    state: await invoke<object>("state").catch(err => console.error(err)),
    ui: await invoke<object>("ui").catch(err => console.error(err)),
    registrate_handler: handlerRegistration,
    throw_event: eventThrower,
  };

  // TODO: Figure out a way to sort modules by runtime
  const modules = Array.from(window.__nmideConfig__.modules.values());
  // TODO: Add proper validation/handling
  const modifications: CoreModification[] = await Promise.all(
    modules.map(m => m.init(core))
  );
  return modifications;
}

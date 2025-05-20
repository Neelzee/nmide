import {
  Core,
  Event,
  CoreModification,
  emptyState,
  State,
  HtmlBuilder,
  Html,
  getEventName,
} from "@nmide/js-utils";
import "./app";
import client from "@nmide/js-client";
import * as E from "fp-ts/Either";
import { pipe } from "fp-ts/lib/function";
import { emit } from "@tauri-apps/api/event";

const registerHandler = async (
  module: string,
  event_name: string,
) => {
  window.__nmideConfig__.log.info(`Module: ${module} -> ${event_name}`);
  let list = window.__nmideConfig__.handlerRegister.event.get(event_name)
  list = list === undefined ? [] : list;
  list.push(module);
  window.__nmideConfig__.handlerRegister.event.set(event_name, list);
};

const eventThrower = async (event: Event) => {
  window.__nmideConfig__.log.debug(`[Frontend] throwing event: ${JSON.stringify(event)}`);
  emit(
    "nmide://event",
    { event }
  ).catch(
    err =>
      window.__nmideConfig__.log.error(
        `Event ${JSON.stringify(event)} resulted in error from backend: ${err} ${JSON.stringify(err)}`, err
      )
  );
};

const sendModification = async (modification: CoreModification) => {
  window.__nmideConfig__.log.debug(`[Frontend] sending modification: ${JSON.stringify(modification)}`);
  client(
    "modification",
    { modification }
  ).catch(
    err =>
      window.__nmideConfig__.log.error(
        `Modification ${JSON.stringify(modification)} resulted in error from backend: ${err} ${JSON.stringify(err)}`, err
      )
  );
};

const mkCore = async (): Promise<Core> => {
  return {
    state: pipe(
      await client("state"),
      E.getOrElse((_) => emptyState()),
      st => () => new Promise<State>(r => r(st))
    ),
    ui: pipe(
      await client("ui"),
      E.getOrElse((_) => new HtmlBuilder().build()),
      st => () => new Promise<Html>(r => r(st))
    ),
    registerHandler,
    eventThrower,
    sendModification,
  };
}

const tsHandler = async (evt: Event) => {
  const core: Core = await mkCore();

  const event_modules = window.__nmideConfig__.handlerRegister.event.get(getEventName(evt));
  const modules = event_modules === undefined ? [] : event_modules;
  window.__nmideConfig__.log.info(`Event: ${JSON.stringify(evt)}, Modules: ${JSON.stringify(modules)}`);
  const modifications: CoreModification[] = await Promise.all(
    modules
      .map(m => window.__nmideConfig__.modules.get(m))
      .filter(m => m !== undefined)
      .map(m => m.handler(evt, core))
  );

  return modifications;
}

const tsInit = async () => {
  const core: Core = await mkCore();

  const modules = Array.from(window.__nmideConfig__.modules.values());
  const modifications: CoreModification[] = await Promise.all(
    modules.map(m => m.init(core))
  );
  return modifications;
}

const TSRuntime = {
  init: tsInit,
  handler: tsHandler
};

export default TSRuntime;

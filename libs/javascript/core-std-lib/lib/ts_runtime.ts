import {
  type Core,
  type Event,
  type CoreModification,
  emptyState,
  type State,
  HtmlBuilder,
  type Html,
  getEventName,
} from "@nmide/js-utils";
import "./app";
import client from "@nmide/js-client";
import * as E from "fp-ts/Either";
import { pipe } from "fp-ts/lib/function";
import { emit } from "@tauri-apps/api/event";
import { DCoreModification, prettyReport } from "@nmide/js-decoder-lib";
import { STATE_INVOKER, UI_INVOKER } from "./nmideConstants";

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
  if (window.__nmideConfig__.installed) {
    emit(
      "nmide://event",
      { event }
    ).catch(
      err =>
        window.__nmideConfig__.log.error(
          `Event ${JSON.stringify(event)} resulted in error from backend: ${err} ${JSON.stringify(err)}`, err
        )
    );
  } else {
    window.__nmideConfig__.events.push(event);
  }
};

const sendModification = async (modification: CoreModification) => {
  const result = DCoreModification.decode(modification);
  if (E.isLeft(result)) {
    const issues = prettyReport.report(result);
    window.__nmideConfig__
      .log
      .debug(
        `[frontend] invalid modification: ${JSON.stringify(modification)} `
        + ` errors: ${issues.join("\n")}`
      );
    return;
  } else {
    window.__nmideConfig__.log.debug(`[frontend] sending modification: ${JSON.stringify(modification)}`);
  }
  client(
    "modification",
    { modification }
  ).then(err => {
    if (E.isLeft(err)) {
      const error = err.left;
      window.__nmideConfig__
        .log
        .error(
          `[frontend] client modification ${JSON.stringify(modification)}`
          + ` resulted in error from backend: ${error}`
          + ` ${JSON.stringify(error)}`
          ,
          error
        )
    }
  })
    .catch(
      err =>
        window.__nmideConfig__.log.error(
          `[frontend] Modification ${JSON.stringify(modification)} resulted in error from backend: ${err} ${JSON.stringify(err)}`, err
        )
    );
};

export const mkCore = async (): Promise<Core> => {
  return {
    state: pipe(
      await client(STATE_INVOKER),
      // NOTE: This hides possible errors
      E.getOrElse((_) => emptyState()),
      st => () => new Promise<State>(r => r(st))
    ),
    ui: pipe(
      await client(UI_INVOKER),
      // NOTE: This hides possible errors
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

  const event_modules = window.__nmideConfig__
    .handlerRegister
    .event.get(getEventName(evt));
  const modules = event_modules === undefined ? [] : event_modules;
  window.__nmideConfig__
    .log
    .info(`Event: ${JSON.stringify(evt)}, Modules: ${JSON.stringify(modules)}`);
  await Promise.all(
    modules
      .map(m => window.__nmideConfig__.modules.get(m))
      .filter(m => m !== undefined)
      .map(m => m.handler(evt, core))
  ).catch(err =>
    window.__nmideConfig__
      .log
      .error(`Handler Error: ${err}, ${JSON.stringify(err)}`)
  );
}

const tsInit = async () => {
  const core: Core = await mkCore();

  const modules = Array.from(window.__nmideConfig__.modules.values());
  await Promise.all(
    modules.map(m => m.init(core))
  ).catch(err =>
    window.__nmideConfig__
      .log
      .error(`Init Error: ${err}, ${JSON.stringify(err)}`)
  );
}

const TSRuntime = {
  init: tsInit,
  handler: tsHandler
};

export default TSRuntime;

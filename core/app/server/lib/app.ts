import {
  defaultConfig,
  NMIDE_INITIALIZED,
} from "@nmide/js-core-std-lib";
import { renderer } from "../../lib/tsRenderer.ts";
import * as E from "fp-ts/Either";
import {
  type Core,
  type Event,
  type CoreModification,
  emptyState,
  type State,
  HtmlBuilder,
  type Html,
  getEventName,
  AppConfig,
} from "@nmide/js-utils";
import { DCoreModification, prettyReport } from "@nmide/js-decoder-lib";
import { applyStateInstr, applyUiInstr } from "@nmide/js-utils/lib/InstructionHelper.ts";

export const registerHandler = async (
  module: string,
  event_name: string,
) => {
  window.__nmideConfig__.log.info(`Module: ${module} -> ${event_name}`);
  let list = window.__nmideConfig__.handlerRegister.event.get(event_name)
  list = list === undefined ? [] : list;
  list.push(module);
  window.__nmideConfig__.handlerRegister.event.set(event_name, list);
};

export const eventThrower = async (event: Event) => {
  window.__nmideConfig__.log.info(`[frontend] throwing event: ${event}`);
  if (window.__nmideConfig__.installed) {
    await handlers(event);
  } else {
    window.__nmideConfig__.events.push(event);
  }
};

export const sendModification = async (modification: CoreModification) => {
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

  const stateMod = modification.state;
  const uiMod = modification.ui;

  await window.__nmideConfig__.render(uiMod);

  state = applyStateInstr(stateMod)(state);
  ui = applyUiInstr(uiMod)(ui);

};

let state: State = emptyState();

let ui: Html = new HtmlBuilder().build();

export const mkCore = async (): Promise<Core> => {
  return {
    state: () => new Promise(r => r(state)),
    ui: () => new Promise(r => r(ui)),
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

const initializers = async () => {
  const promises = [];
  for (let i = 0; i < window.__nmideConfig__.runtimes.initializers.length; i++) {
    const init = window.__nmideConfig__.runtimes.initializers[i];
    promises.push(init());
  }
}

const handlers = async (event: Event) => {
  const promises = [];
  for (let i = 0; i < window.__nmideConfig__.runtimes.handlers.length; i++) {
    const handler = window.__nmideConfig__.runtimes.handlers[i];
    promises.push(handler(event));
  }
}


export const serverRuntime = {
  init: tsInit,
  handler: tsHandler
};

const App = {
  initialize: async (config: Partial<AppConfig> = {}) => {
    const render = renderer(eventThrower);
    window.__nmideConfig__ = { ...defaultConfig, ...config, render }
    document.dispatchEvent(new CustomEvent(NMIDE_INITIALIZED));
  },
  run: async () => {
    await initializers()
      .catch(console.error)
      .finally(() => {
        window.__nmideConfig__.installed = true;
        (async () => {
          window.__nmideConfig__.log.info("[frontend] sending events:", window.__nmideConfig__.events);
          const promises = window.__nmideConfig__.events.map(event => {
            handlers(event)
          });
          await Promise.all(promises);
          window.__nmideConfig__.events = [];
        })();
      });
  }
};

export default App;
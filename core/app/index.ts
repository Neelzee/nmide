import { invoke } from "@tauri-apps/api/core";
import { emit, listen } from "@tauri-apps/api/event";
import { TUIInstruction } from "@nmide/js-utils/lib/Html";
import { Module, TEvent, THtml } from "@nmide/js-utils";
import {
  Core,
  Event,
  state_from_obj,
  combine_modifications
} from "lib_gleam";
import { ideInstallModules } from "./lib/ideInstallModules.ts";
import { AppConfig, defaultConfig } from "./App.ts"
import {
  NMIDE_INITIALIZED,
  NMIDE_MODULES_INSTALLED_EVENT
} from "./nmideConstants.ts";
import { run } from "./main.ts";
import { eventThrower } from "./lib/eventThrower.ts";
import { handlerRegistration } from "./lib/handlerRegistration.ts";
import { tsInit, tsHandler } from "./tsRuntime.ts";

run({
  initialize : (config: Partial<AppConfig> = {}) => {
    const conf = { ...defaultConfig, ...config };
    // @ts-expect-error This is okay
    window.__nmideConfig__ = {}
    // @ts-expect-error This is okay
    Object.keys(conf).forEach(key => { window.__nmideConfig__[key] = conf[key] });

    /* HACK: This makes it so that we get an accurate count of how many modules
     * are installed, since they install asynchronously, we have to "wait" until
     * the count is 0
    */
    const originalSetter =
      window.__nmideConfig__.modules
        .set.bind(window.__nmideConfig__.modules);
    window.__nmideConfig__.modules.set = (key: string, val: Module) => {
      const res = originalSetter(key, val);
      console.log("Setting")
      window.__nmideConfig__.moduleCount--;
      if (window.__nmideConfig__.moduleCount === 0 && !window.__nmideConfig__.installed) {
        window.__nmideConfig__.installed = true;
        document.dispatchEvent(
          new CustomEvent(NMIDE_MODULES_INSTALLED_EVENT)
        );
      }
      return res;
    };
    document.dispatchEvent(new CustomEvent(NMIDE_INITIALIZED));
  },
  installModules : () => {
    ideInstallModules().then(() => {
      if (window.__nmideConfig__.moduleCount === 0 && !window.__nmideConfig__.installed) {
        window.__nmideConfig__.installed = true;
        document.dispatchEvent(
          new CustomEvent(NMIDE_MODULES_INSTALLED_EVENT)
        );
      }
    }).catch(err => {
      window.__nmideConfig__.log.error("Error on installation: ", err)
      if (window.__nmideConfig__.moduleCount === 0 && !window.__nmideConfig__.installed) {
        window.__nmideConfig__.installed = true;
        document.dispatchEvent(
          new CustomEvent(NMIDE_MODULES_INSTALLED_EVENT)
        );
      }
    });
  },
  run: () => {
    listen<TUIInstruction["ui"]>("nmide://render", ({ payload: obj }) => {
      window.__nmideConfig__.render({ ui: obj })
        .catch(err => window.__nmideConfig__.log.error("Error on render: ", err));
    }).catch((err) => window.__nmideConfig__.log.error("nmide://render", err));

    listen<TEvent>("nmide://event", ({ payload: event }) => {
      invoke<object>("state")
        .then(state => invoke<THtml>("ui").then(ui => {
          return new Core(state_from_obj(deObjectify(state)), ui, eventThrower, handlerRegistration);
        }).catch(err => window.__nmideConfig__.log.error("Invocation: ", err)))
        .then(async core => {
        const promises = [];
        for (let i = 0; i < window.__nmideConfig__.runtimes.handlers.length; i++) {
          const init = window.__nmideConfig__.runtimes.handlers[i];
          promises.push(init(new Event(event.eventName, event.moduleName, event.args), core));
        }
        return combine_modifications(await Promise.all(promises));
      }).then(_cm =>
        invoke<TUIInstruction["op"]>("handler")
          .then((op) => {
            window.__nmideConfig__.render({ op })
              .catch(err => window.__nmideConfig__.log.error("Error on render: ", err));
          })
          .catch((err) => console.error("Handler: ", err))
      );
    }).catch((err) => window.__nmideConfig__.log.error("nmide://render", err));

    (async () => {
        const promises = [];
        for (let i = 0; i < window.__nmideConfig__.runtimes.initializers.length; i++) {
          const init = window.__nmideConfig__.runtimes.initializers[i];
          promises.push(init());
        }
        return combine_modifications(await Promise.all(promises));
      })()
      .catch(
        err =>
          window.__nmideConfig__.log.error("Initialization error: ", err)
      )
      .then(cm => {
        if (typeof cm !== "object") {
          return { state: "noOp", ui: ["noOp", "noOp", "noOp"] };
        }
        return cm;
      })
      .then(cm =>
      invoke<TUIInstruction["ui"]>("init", { state: cm.state, ui: cm.ui })
        .then((op) => {
          window.__nmideConfig__.render({ ui: op })
            .catch(err => window.__nmideConfig__.log.error("Error on render: ", err));
        })
        .catch((err) => console.error("Init: ", err))
    );
  }
},
  {
    runtimes:
      {
        handlers: [tsHandler],
        initializers: [tsInit]
      },
  }
);


const deObjectify = (o: object): [string, unknown][] => {
  return Object.entries(o).map(([k, v]) => {
    if (typeof v === "object") {
      return [k, deObjectify(v)];
    } else {
      return [k, v];
    }
  });
}
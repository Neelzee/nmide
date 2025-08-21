import {
  allAttrs,
  getEvent,
  getEventName,
  HtmlBuilder,
  mkPrimEvent,
  StateBuilder,
  type Core,
  type CoreModification,
  type Event,
  type Html,
  type Module,
  type State
} from "@nmide/js-utils";
import { flatten, isAdd } from "@nmide/js-utils/lib/InstructionHelper";
import * as fs from "fs";
import { type NamedDependency } from "../rsm-grapher/rsm-invoker/bindings/NamedDependency";

// Module -> Event
const provider: Map<string, Event[]> = new Map();

// Event -> Module
const consumer: Map<string, string[]> = new Map();

const init_modules: NamedDependency[] = JSON.parse(fs.readFileSync("./intermediate_result.json").toString());

const originalSetter =
  window.__nmideConfig__.modules
    .set.bind(window.__nmideConfig__.modules);

//@ts-expect-error Is only undefined on exit
window.__nmideConfig__.modules.set = (key: string, val: Module) => {
  const core: Core = {
    ui: async (): Promise<Html> => new HtmlBuilder().build(),
    state: async (): Promise<State> => new StateBuilder().toState(),
    eventThrower: async (evt: Event): Promise<void> => {
      console.log(key, JSON.stringify(evt));
      const events = provider.get(key) || [];
      events.push(evt);
      provider.set(key, events);
    },
    registerHandler: async (name: string, event: string): Promise<void> => {
      console.log(name, event);
      const modules = consumer.get(event) || [];
      modules.push(name);
      consumer.set(event, modules);
    },
    sendModification: async (modification: CoreModification): Promise<void> => {
      console.log(key, JSON.stringify(modification));
      const [ui, _, attrs] = modification.ui;
      const events = flatten(ui)
        .filter(isAdd)
        .map(i => i.add[1])
        .flatMap(allAttrs)
        .map(getEvent)
        .filter(x => x !== undefined);

      events.push(
        ...flatten(attrs)
          .filter(isAdd)
          .map(a => a.add[1])
          .map(getEvent)
          .filter(x => x !== undefined)
      );

      events
        .forEach(evt => {
          const events = provider.get(key) || [];
          events.push(evt);
          provider.set(key, events);
        });

    }
  }

  let res;
  let err;

  try {
    res = originalSetter(key, val);
  } catch (err) {
    err = err;
    val.init(core)
      .catch(console.error)
      .then(() => sleep(10_000))
      .then(() => {
        console.error(err);
        console.log("Logs: ", logs);
        console.log("Providers: ", provider);
        console.log("Consumer: ", consumer);
      });
  }

  init_modules
    .find(({ name }) => name === key)?.consuming
    .forEach(({ event_name }) => {
      const event: Event = init_modules.find(
        ({ providing }) => providing.find(e => getEventName(e) === event_name) !== undefined
      )?.providing.find(e => getEventName(e) === event_name) || mkPrimEvent(event_name);
      val.handler(event, core).catch(console.error);
    });

  return res;
};

import "./setup";
import "../build/modules"; import { sleep } from "bun";
import { logs } from "./setup";

document.dispatchEvent(new window.CustomEvent("nmide://ModulesInstalled"));

await sleep(45_000)
  .then(() => {
    console.log("Logs: ", logs);
    console.log("Providers: ", provider);
    console.log("Consumer: ", consumer);

    // Module -> Event
    const provider_obj = Array.from(provider.entries());
    // Event -> Module
    const consumer_obj = Array.from(consumer.entries());

    const init_m: Map<string, Omit<NamedDependency, "name">> = new Map();

    init_modules.forEach(m => {
      init_m.set(m.name, {
        providing: m.providing,
        consuming: m.consuming,
        success: m.success
      });
    });

    Array.from(window.__nmideConfig__.modules.entries()).map(([name, _]) => {
      return {
        name,
        providing: provider.get(name) || [],
        consuming: Array.from(consumer.entries())
          .filter(([_, xs]) => xs.includes(name))
          .map(([event_name, _]) => {
            return { event_name };
          }),
        success: true,
      };
    }).forEach(m => {
      const module = init_m.get(m.name);
      if (module === undefined) {
        init_m.set(m.name, {
          // @ts-expect-error This should be valid
          providing: m.providing,
          consuming: m.consuming,
          success: m.success
        });
      } else {
        // @ts-expect-error This should be valid
        module.providing.push(...m.providing.filter(e => !module.providing.includes(e)));
        module.consuming.push(...m.consuming.filter(e => !module.consuming.includes(e)));
        init_m.set(m.name, module);
      }
    });

    const modules: NamedDependency[] = Array.from(init_m.entries()).map(([name, obj]) => {
      return { name, ...obj };
    })

    fs.writeFileSync("../build/jsm_result.json", JSON.stringify(modules));
  });

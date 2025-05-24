import {
  allAttrs,
  getEvent,
  HtmlBuilder,
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

// Module -> Event
const provider: Map<string, Event[]> = new Map();

// Event -> Module
const consumer: Map<string, string[]> = new Map();

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

  val.init(core)
    .catch(console.error)
    .then(() => sleep(10_000))
    .then(() => {
      console.log("Finished: ", key);
    });


  return res;
};

import "./setup";
import "../build/modules"; import { sleep } from "bun";
import { logs } from "./setup";

document.dispatchEvent(new window.CustomEvent("nmide://ModulesInstalled"));

await sleep(15_000)
  .then(() => {
    console.log("Logs: ", logs);
    console.log("Providers: ", provider);
    console.log("Consumer: ", consumer);

    // Module -> Event
    const provider_obj = Array.from(provider.entries());
    // Event -> Module
    const consumer_obj = Array.from(consumer.entries());

    const modules = Array.from(window.__nmideConfig__.modules.entries()).map(([name, _]) => {
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
    });
    fs.writeFileSync("./intermediate_result.json", JSON.stringify(modules));
  });


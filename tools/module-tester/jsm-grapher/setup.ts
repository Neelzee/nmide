import { JSDOM } from "jsdom";

type DebugConfig = {
  log: {
    error: (...args: unknown[]) => void,
    debug: (...args: unknown[]) => void,
    info: (...args: unknown[]) => void,
  },
  modules: Map<string, Module>
};

declare global {
  var window: Window & typeof globalThis;
  namespace NodeJS {
    interface Global {
      window: Window & {
        __nmideConfig__: DebugConfig,
      };
      document: any,
    }
  }
}

const dom = new JSDOM();
// @ts-expect-error This is valid for testing
global.window = dom.window;
global.document = dom.window.document;

import type { Module } from "@nmide/js-utils";

export const logs: Map<string, unknown[]> = new Map();

export const createMockConfig = (): DebugConfig => {
  return {
    modules: new Map(),
    log: {
      error: (...args) => {
        const l = logs.get("error") || [];
        l.push(args);
        logs.set("error", l);
      },
      debug: (...args) => {
        const l = logs.get("debug") || [];
        l.push(args);
        logs.set("debug", l);
      },
      info: (...args) => {
        const l = logs.get("info") || [];
        l.push(args);
        logs.set("info", l);
      },
    },
  };
};

// @ts-expect-error This should be enough for debugging.
global.window.__nmideConfig__ = createMockConfig();


const originalSetter =
  window.__nmideConfig__.modules
    .set.bind(window.__nmideConfig__.modules);
window.__nmideConfig__.modules.set = (key: string, val: Module) => {
  const res = originalSetter(key, val);

  window.__nmideConfig__.moduleCount--;
  if (window.__nmideConfig__.moduleCount === 0 && !window.__nmideConfig__.installed) {
    window.__nmideConfig__.installed = true;
    throw Error("Finished");
  }
  return res;
};

import {
  click,
  HtmlBuilder,
  id,
  installModule,
  isPrimAnd,
  isPrimitiveEvent,
  isTBool,
  mkPrimEvent,
  StateBuilder,
  tBool,
  UiBuilder,
  type Core,
  type Event,
  type Module,
  type Value
} from "@nmide/js-utils"
import { pipe } from "fp-ts/lib/function";
import { fst, snd } from "fp-ts/lib/Tuple";
import * as A from "fp-ts/Array";

const name = "module-form";

const getQueryParamsFromModules = (modules: [string, boolean][]): string =>
  modules.length === 0
    ? ""
    : pipe(
      modules,
      A.filter(snd),
      A.map(fst),
      A.map(m => `module=${m}`),
      xs => xs.join("&"),
      s => `?${s}`,
    );

const mkSafeModuleName = (m: string): string =>
  `module-${JSON.stringify(m).replace("\"", "").replace("\"", "")}`;


const module: Module = {
  name,
  init: async (core: Core): Promise<void> => {
    await core.registerHandler(name, "form-submit");
    const modules = Array.from(window.__nmideConfig__.modules.entries())
      .map(fst)
      .filter(m => m != name)
      .map(m => {
        const safeModule = mkSafeModuleName(m)
        return [new HtmlBuilder()
          .kind("label")
          .attrs(
            { custom: ["for", safeModule] }
          )
          .text(m),
        new HtmlBuilder()
          .kind("input")
          .attrs(
            { custom: ["type", "checkbox"] },
            { custom: ["name", safeModule] },
            click(mkPrimEvent(safeModule))
          )
        ];
      });

    await core.sendModification(
      new UiBuilder()
        .add(
          new HtmlBuilder()
            .kind("form")
            .attrs(id("module-form"))
            .kids(
              ...(modules.flatMap(x => x)),
              new HtmlBuilder()
                .kind("button")
                .text("Submit")
                .attrs(
                  id("form-submit"),
                  click(mkPrimEvent("form-submit")),
                )
            )
        )
        .build()
    );
  },
  handler: async (event: Event, core: Core): Promise<void> => {
    if (isPrimAnd(event, "form-submit")) {
      const state = await core.state();
      const modules: [string, boolean][] = pipe(
        Array.from(window.__nmideConfig__.modules.entries()),
        A.map(fst),
        A.filter(m => m != name),
        A.map(mkSafeModuleName),
        A.map<string, [string, Value | undefined]>((m) => [m, state[m]]),
        A.filter<[string, Value | undefined], [string, Value]>((x): x is [string, Value] => snd(x) !== undefined),
        A.map(([m, v]) => [m, isTBool(v) ? v.bool : false])
      );
      // Redirect the user based on the state
      window.location.replace(`https://${window.location.hostname}/form.index${getQueryParamsFromModules(modules)}`);
    } else if (isPrimitiveEvent(event)) {
      const { event: { event: moduleName } } = event;
      const state = await core.state();
      const oldValue = state[moduleName] || tBool(true);
      const newValue = isTBool(oldValue) ? oldValue.bool : false;
      await core.sendModification(
        new StateBuilder().set(moduleName, tBool(newValue)).build()
      );
    }
  }
};

installModule(module);

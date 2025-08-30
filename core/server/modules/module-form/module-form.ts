import { click, custom, getArgs, HtmlBuilder, id, installModule, isTObj, mkPrimEvent, UiBuilder, type Core, type Event, type Module } from "@nmide/js-utils"
import { pipe } from "fp-ts/lib/function";
import { fst } from "fp-ts/lib/Tuple";

const name = "module-form";

const module: Module = {
  name,
  init: async (core: Core): Promise<void> => {
    await core.registerHandler(name, "form-submit");
    const modules = Array.from(window.__nmideConfig__.modules.entries())
      .filter(([k, v]) => k != name)
      .map(fst)
      .map(m => {
        const safeModule = `module-${JSON.stringify(m).replace("\"", "").replace("\"", "")}`;
        console.log(`'${safeModule}'`)
        return new HtmlBuilder()
          .kind("label")
          .attrs(
          //custom("for", safeModule)
        )
          .text(m)
          .kids(
            new HtmlBuilder()
              .kind("input")
              .attrs(
                custom("type", "checkbox"),
                //      custom("name", safeModule)
              )
          );
      });

    await core.sendModification(
      new UiBuilder()
        .add(
          new HtmlBuilder()
            .kind("form")
            .attrs(id("module-form"))
            .kids(
              ...modules,
              new HtmlBuilder()
                .kind("button")
                .text("Submit")
                .attrs(
                  id("form-submit"),
                  custom("type", "submit"),
                  click(mkPrimEvent("form-submit")),
                )
            )
        )
        .build()
    );
  },
  handler: async (event: Event, core: Core): Promise<void> => {
    const args = getArgs(event);
    console.log(event)
    if (isTObj(args)) {
      const obj = args.obj;
      const formValue = obj["form"];
      if (isTObj(formValue)) {
        const form = pipe(
          Object.entries(formValue.obj),
          a => {
            console.log(a);
            return a;
          }
        );
      }
    }
  }
};

installModule(module);

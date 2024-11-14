import { HtmlBuilder, THtml, TMap, TMsg } from "@nmide/js-utils"

// @ts-ignore
window.plugins.set(
  "DependencyViewer",
  {
    init: (): TMap => {
      return [];
    },
    view: (model: TMap): THtml => {
      return new HtmlBuilder()
        .kids([
          new HtmlBuilder()
            .kind("Div")
            .attrs([{ Id: "container" }]),
        ])
        .build();
    },
    update: (msg: TMsg, model: TMap): TMap => {
      return [];
    },
  }
);

import { getValue, HtmlBuilder, THtml, tList, tLookup, TMap, TMsg, tObj, TValueBool, TValueObj, TValueStr } from "@nmide/js-utils"
import MapBuilder from "@nmide/js-utils/lib/MapBuilder";
import { map, getOrElse } from "fp-ts/Option";
import { pipe } from "fp-ts/lib/function";

// @ts-ignore
window.plugins.set(
  "DependencyViewer",
  {
    init: (): TMap => {
      return new MapBuilder()
        .add("DependencyViewerInit", false)
        .build();
    },
    view: (model: TMap): THtml => {
      return new HtmlBuilder()
        .kids([
          new HtmlBuilder()
            .kind("Button")
            .text("Render Dependency")
            .attrs([{
              OnClick: {
                Msg: ["dependency_render", {
                  Str: pipe(
                    model,
                    tLookup<TValueStr>("info-module-graph"),
                    map<TValueStr, string>(a => a.Str),
                    getOrElse(() => {
                      console.log("Module does not have info-module-graph: ", model);
                      return "";
                    })
                  )
                }]
              }
            }])
        ])
        .build();
    },
    update: (msg: TMsg, model: TMap): TMap => {
      if (msg.Msg[0] !== "dependency_render") return [];
      const skip = pipe(
        model,
        tLookup<TValueBool>("DependencyViewerInit"),
        map<TValueBool, boolean>(a => a.Bool),
        getOrElse(() => false),
      );
      if (skip) return [];
      const graph = pipe(
        model,
        tLookup<TValueObj>("info-module-graph"),
        getOrElse(() => tObj([])),
      );
      const div = document.createElement("div");
      div.id = "graph";
      document.body.appendChild(div);
      //@ts-ignore
      window.plugins.get("DependencyViewerHelper").render(graph);
      return [];
    },
  }
);

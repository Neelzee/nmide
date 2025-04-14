import {
  THtml,
  TMap,
  TMsg,
  MapBuilder,
  emptyHtml,
  TValueStr,
  tStr,
  tLookupOr
} from "@nmide/js-utils"

//@ts-ignore
window.__nmideConfig__.modules.set(
  "DependencyViewer",
  {
    name: "DependencyViewer",
    init: (_: unknown): TMap => {
      return new MapBuilder()
        .add("DependencyViewerInit", false)
        .build();
    },
    view: (model: TMap): THtml => {
      const graph = tLookupOr<TValueStr>("info-module-graph")(tStr(""))(model);
      let div = document.getElementById("graph");
      if (div === null) div = document.createElement("div");
      div.id = "graph";
      div.textContent = "";
      window.root.appendChild(div);
      window.plugins.get("DependencyViewerHelper").render(graph.str);
      return emptyHtml();
    },
    update: (_: TMsg, __: TMap): TMap => [],
  }
);

import {
  Core,
  CoreModification,
  emptyCm,
  Event,
  HtmlBuilder,
  id,
  installModule,
  UiBuilder
} from "@nmide/js-utils";
installModule(
  {
    name: "DependencyViewer",
    init: async (core: Core): CoreModification => {
      const script = document.createElement("script");
      script.type = "text/javascript";
      script.src = "https://cdn.jsdelivr.net/npm/d3@7"
      document.head.appendChild(script);
      await core.registerHandler("DependencyViewer", "graph")
        .catch(err => console.error("error from module: ", err));
      return new UiBuilder()
        .add(new HtmlBuilder().kind("svg").attrs(id("graph")))
        .build();
    },
    handler: async ({ event }: Event, __: Core) => {
      if (event === "graph") {
        console.log("Graphing");
      }
      return emptyCm();
    }
  }
);

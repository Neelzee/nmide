import {
  Core,
  CoreModification,
  emptyCm,
  Event, HtmlBuilder,
  installModule, UiBuilder
} from "@nmide/js-utils";

const moduleName = "ide-editor";

installModule(
  {
    name: moduleName,
    init: async (core: Core): CoreModification => {
      await core.registerHandler(moduleName, "editor-click", null)
        .catch(err => console.error(moduleName, err));
      return new UiBuilder()
        .add(
          new HtmlBuilder()
            .attrs({ id: "editor-div" })
            .kids(
              new HtmlBuilder()
                .kind("textArea")
                .attrs(
                  { id: "editor-area" },
                  { click: { event: "editor-click", module: moduleName, args: null } },
                )
            ),
        )
        .build();
    },
    handler: async (event: Event, _: Core): CoreModification => {
      if (event.event === "editor-click") {
        console.log("editor: ", event);
      }
      return emptyCm();
    }
  }
);

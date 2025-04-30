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
    init: async (core: Core): Promise<CoreModification> => {
      await core.registerHandler(moduleName, "editor-click")
        .catch(err => console.error(moduleName, err));
      await core.registerHandler(moduleName, "post-init")
        .catch(err => console.error(moduleName, err));
      return emptyCm();
    },
    handler: async (event: Event, _: Core): Promise<CoreModification> => {
      if (event.event === "editor-click") {
        console.log("editor: ", event);
      }
      if (event.event === "post-init") {
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
              "root"
          )
          .build();
      }
      return emptyCm();
    }
  }
);

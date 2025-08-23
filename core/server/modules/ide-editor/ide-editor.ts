import {
  click,
  cls,
  Core,
  Event, HtmlBuilder,
  installModule, isPrimAnd, mkPrimEvent, UiBuilder
} from "@nmide/js-utils";

const moduleName = "ide-editor";

installModule(
  {
    name: moduleName,
    init: async (core: Core): Promise<void> => {
      await core.registerHandler(moduleName, "editor-click")
        .catch(err => console.error(moduleName, err));
      await core.registerHandler(moduleName, "open-editor-area")
        .catch(err => console.error(moduleName, err));
    },
    handler: async (event: Event, core: Core): Promise<void> => {
      if (isPrimAnd(event, "open-editor-area")) {
        await core.eventThrower(mkPrimEvent("add_content",
          {
            html: new HtmlBuilder()
              .attrs(
                { id: "editor-div" },
                cls("editor-container")
              )
              .kids(
                new HtmlBuilder()
                  .kind("textArea")
                  .attrs(
                    { id: "editor-area" },
                    click(mkPrimEvent("editor-click"))
                  )
              ).build()
          }
        ));
      }
    }
  }
);

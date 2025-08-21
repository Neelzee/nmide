import {
  HtmlBuilder,
  id,
  mkPrimEvent,
  UiBuilder,
  type Core,
  type Event,
  type Module
} from "@nmide/js-utils";
import { NOTIFICATION_ID, POST_FRAMEWORK_EVENT } from "../lib/constants";

const name = "f-squared";

const FrameworkModule: Module = {
  name,
  init: async (core: Core): Promise<void> => {
    await core.sendModification(
      new UiBuilder()
        .add(
          new HtmlBuilder()
            .kind("main")
            .attrs(id("root"))
            .kids(
              new HtmlBuilder().attrs(id("navbar")),
              new HtmlBuilder()
                .attrs(id("sidebar"))
                .kids(
                  new HtmlBuilder()
                    .attrs(id("project"))
                    .kids(
                      new HtmlBuilder().kind("span").attrs(id("project-title")),
                      new HtmlBuilder().kind("ol").attrs(id(NOTIFICATION_ID))
                    ),
                ),
              new HtmlBuilder().attrs(id("content"))
            )
        )
        .build()
    );
    await core.eventThrower(mkPrimEvent(POST_FRAMEWORK_EVENT));
  },
  handler: async (_: Event, __: Core): Promise<void> => { }
}

export default FrameworkModule;

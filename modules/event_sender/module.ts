import {
  click,
  cls,
  emptyCm,
  getEventName,
  HtmlBuilder,
  id,
  mkPrimEvent,
  UiBuilder,
  type Core,
  type CoreModification,
  type Event
} from "@nmide/js-utils";

let toggle = false;

const Module = {
  name: "event_sender",
  init: async (core: Core): Promise<CoreModification> => {
    core.registerHandler("event_sender", "toggle-debug")
      .catch(console.error);
    const ui = new HtmlBuilder()
      .attrs(id("debug"))
      .kids(
        new HtmlBuilder()
          .kind("button")
          .attrs(click(mkPrimEvent("toggle-debug")))
          .text("Debug"),
        new HtmlBuilder()
          .attrs(id("debug-content"))
          .kids(
            new HtmlBuilder()
              .attrs(id("debug-event-selector"))
              .kids(
                new HtmlBuilder()
                  .kids(
                    new HtmlBuilder()
                      .kind("label")
                      .text("Event: ")
                      .attrs({ custom: ["for", "event-name"] }),
                    new HtmlBuilder()
                      .kind("select")
                      .attrs({ custom: ["name", "event-name"] })
                      .kids(
                        ...["Event", "post-init", "pre-exit", "Dialog", "Dialog File", "Core Response"]
                          .map(e => new HtmlBuilder().kind("option").text(e))
                      )
                  )
              ),
            // Post-Init Event
            new HtmlBuilder()
              .kind("form")
              .kids(
                new HtmlBuilder()
                  .kind("label")
                  .text("Post Init: ")
                  .attrs({ custom: ["for", "event-name"] }),
                new HtmlBuilder()
                  .kind("input")
                  .attrs(
                    { custom: ["name", "event-name"] },
                    { custom: ["disabled", ""] },
                    { custom: ["value", "post-init"] },
                  ),
                new HtmlBuilder()
                  .kind("button")
                  .text("Send")
                  .attrs(click({ event: { event: "post-init", args: null } })),
              ),
            // Pre-Exit Event
            new HtmlBuilder()
              .kind("form")
              .kids(
                new HtmlBuilder()
                  .kind("label")
                  .text("Pre Exit: ")
                  .attrs({ custom: ["for", "event-name"] }),
                new HtmlBuilder()
                  .kind("input")
                  .attrs(
                    { custom: ["name", "event-name"] },
                    { custom: ["disabled", ""] },
                    { custom: ["value", "pre-exit"] },
                  ),
                new HtmlBuilder()
                  .kind("button")
                  .text("Send")
                  .attrs(click({ event: { event: "pre-exit", args: null } })),
              ),
            // Event
            new HtmlBuilder()
              .kind("form")
              .kids(
                new HtmlBuilder()
                  .kind("label")
                  .text("Event: ")
                  .attrs({ custom: ["for", "event-name"] }),
                new HtmlBuilder()
                  .kind("input")
                  .attrs(
                    { custom: ["name", "event-name"] },
                  ),
                new HtmlBuilder()
                  .kind("label")
                  .text("Args: ")
                  .attrs({ custom: ["for", "args"] }),
                new HtmlBuilder()
                  .kind("textarea")
                  .attrs(
                    { custom: ["name", "args"] },
                  ),
                new HtmlBuilder()
                  .kind("button")
                  .text("Send")
                  .attrs(click({ event: { event: "pre-exit", args: null } })),
              )
          )
      )
    return new UiBuilder().add(ui).build();
  },
  handler: async (event: Event, _: Core): Promise<CoreModification> => {
    switch (getEventName(event)) {
      case "toggle-debug":
        return toggle 
          ? new UiBuilder()
            .add_attr(cls("show-debug"), "debug-content").build()
          : new UiBuilder().rem_attr(cls("show-debug"), "debug-content").build()
      default:
        break;
    }
    return emptyCm();
  }
};

export default Module;
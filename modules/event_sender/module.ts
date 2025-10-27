import {
  change,
  click,
  cls,
  HtmlBuilder,
  id,
  isPrimitiveEvent,
  isTBool,
  isTObj,
  mkPrimEvent,
  StateBuilder,
  tBool,
  tObj,
  tObjLookupOr,
  tObjLookupUnd,
  tStr,
  UiBuilder,
  type Core,
  type Event,
  type ValueObj,
  type ValueStr
} from "@nmide/js-utils";

const Module = {
  name: "event_sender",
  init: async (core: Core): Promise<void> => {
    await core.registerHandler("event_sender", "toggle-debug")
      .catch(console.error);

    await core.registerHandler("event_sender", "Event")
      .catch(console.error);

    await core.registerHandler("event_sender", "pre-exit")
      .catch(console.error);

    await core.registerHandler("event_sender", "file dialog event")
      .catch(console.error);

    await core.registerHandler("event_sender", "dialog event")
      .catch(console.error);

    await core.registerHandler("event_sender", "post-init")
      .catch(console.error);
    const ui = new HtmlBuilder()
      .attrs(id("debug"))
      .kids(
        new HtmlBuilder()
          .kind("button")
          .attrs(id("debug-btn"))
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
                      .attrs(
                        { custom: ["name", "event-name"] },
                        change(mkPrimEvent("debug-event-selection"))
                      )
                      .kids(
                        ...["event-id", "post-init-id", "pre-exit-id", "dialog-id", "file-dialog-id", "core-response-id"]
                          .map(e => new HtmlBuilder().kind("option").text(e))
                      )
                  )
              ),
            // Post-Init Event
            new HtmlBuilder()
              .kind("form")
              .attrs(id("post-init-id"))
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
              .attrs(id("pre-exit-id"))
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
              .attrs(
                id("event-id"),
                cls("show-debug-event")
              )
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
                  .kind("textArea")
                  .attrs(
                    { custom: ["name", "args"] },
                  ),
                new HtmlBuilder()
                  .kind("button")
                  .text("Send")
                  .attrs(click({ event: { event: "Event", args: null } })),
              ),
            // Dialog
            new HtmlBuilder()
              .kind("form")
              .attrs(id("dialog-id"))
              .kids(
                new HtmlBuilder()
                  .kind("label")
                  .text("Dialog Event: ")
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
                  .kind("textArea")
                  .attrs(
                    { custom: ["name", "args"] },
                  ),
                new HtmlBuilder()
                  .kind("button")
                  .text("Send")
                  .attrs(click({ event: { event: "dialog event", args: null } })),
              ),
            // Dialog File
            new HtmlBuilder()
              .kind("form")
              .attrs(id("file-dialog-id"))
              .kids(
                new HtmlBuilder()
                  .kind("label")
                  .text("File Dialog Event: ")
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
                  .kind("textArea")
                  .attrs(
                    { custom: ["name", "args"] },
                  ),
                new HtmlBuilder()
                  .kind("button")
                  .text("Send")
                  .attrs(click({ event: { event: "file dialog event", args: null } })),
              ),
            // Core Response
            new HtmlBuilder()
              .kind("form")
              .attrs(id("core-response-id"))
              .kids(
                new HtmlBuilder()
                  .kind("label")
                  .text("Core Response Event: ")
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
                  .kind("textArea")
                  .attrs(
                    { custom: ["name", "args"] },
                  ),
                new HtmlBuilder()
                  .kind("button")
                  .text("Send")
                  .attrs(click({ event: { event: "core response event", args: null } })),
              )
          )
      )
    await core.sendModification(
      new UiBuilder().add(ui)
        .build(new StateBuilder().add("debug-toggle", tBool(false)))
    );
  },
  handler: async (evt: Event, core: Core): Promise<void> => {
    const state = await core.state();
    const value = state["debug-toggle"];
    const toggle = !(isTBool(value) ? value.bool : false);
    const builder = new StateBuilder().add("debug-toggle", tBool(toggle));
    if (!isPrimitiveEvent(evt)) return;
    const { event, args } = evt.event;
    switch (event) {
      case "toggle-debug":
        await core.sendModification(toggle
          ? new UiBuilder()
            .add_attr(cls("show-debug"), "debug-content")
            .build(builder)
          : new UiBuilder()
            .rem_attr(cls("show-debug"), "debug-content")
            .build(builder));
        return;
      case "debug-event-selection":
        const events = ["pre-exit-id", "event-id", "post-init-id", "dialog-id", "file-dialog-id", "core-response-id"];
        const { str: event_selection } = isTObj(args)
          ? tObjLookupOr<ValueStr>("event-name")(tStr("event-id"))(args)
          : tStr("event-id");
        let ui_builder = new UiBuilder().add_attr(cls("show-debug-event"), event_selection);
        events
          .filter(e => e !== event_selection)
          .forEach(e => {
            ui_builder = ui_builder.rem_attr(cls("show-debug-event"), e);
          });
        await core.sendModification(ui_builder.build());
        return;
      case "dialog event":
        {
          if (!isTObj(args)) return;
          const form = tObjLookupOr<ValueObj>("form")(tObj({}))(args);
          core.eventThrower({
            dialogEvent: {
              event: tObjLookupOr<ValueStr>("event-name")(tStr("debug-dialog-event"))(form).str,
              kind: null,
              message: "",
              btn: null,
              title: null
            }
          });
        }
        break;
      case "Event":
        {
          if (!isTObj(args)) return;
          const form = tObjLookupOr<ValueObj>("form")(tObj({}))(args);
          const arg = tObjLookupUnd("args")(form);
          core.eventThrower(mkPrimEvent(
            tObjLookupOr<ValueStr>("event-name")(tStr("debug-event"))(form).str,
            arg === undefined
              ? undefined
              : arg
          )).catch(window.__nmideConfig__.log.error);
        }
        break;
      case "post-init":
      case "pre-exit":
        core.eventThrower(`nmide://${event}`)
          .catch(window.__nmideConfig__.log.error);
        break;
      default:
        break;
    }
  }
};

export default Module;

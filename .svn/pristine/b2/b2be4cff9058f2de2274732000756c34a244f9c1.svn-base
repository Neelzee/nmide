import {
  emptyCm,
  type Module,
  type Core,
  type Event,
  HtmlBuilder,
  id,
  click,
  mkPrimEvent,
  UiBuilder,
  StateBuilder,
  tBool,
  isPrimitiveEvent,
  cls,
  isTBool
} from "@nmide/js-utils"
import { initializeTree } from "./tree";

const updateTree = initializeTree();

const module: Module = {
  name: "state_viz",
  init: async (core: Core) => {
    const script = document.createElement("script");
    script.type = "text/javascript";
    script.src = "https://cdn.jsdelivr.net/npm/d3@7"
    document.head.appendChild(script);
    core.registerHandler("state_viz", "toggle-state-debug")
      .catch(window.__nmideConfig__.log.error);
    const state = new StateBuilder()
      .add("toggle-state-debug", tBool(false))
    const ui = new HtmlBuilder()
      .attrs(id("debug-state"))
      .kids(
        new HtmlBuilder()
          .attrs(id("debug-state-btn-container"))
          .kids(
            new HtmlBuilder()
              .kind("button")
              .text("Debug State")
              .attrs(
                click(mkPrimEvent("toggle-state-debug")),
                cls("debug-button")
              )
          ),
        new HtmlBuilder()
          .attrs(
            id("state-viz"),
            cls("hide-debug-state")
          )
      );

    await core.sendModification(new UiBuilder().add(ui).build(state));
  },
  handler: async (evt: Event, core: Core) => {
    if (!isPrimitiveEvent(evt)) return;
    const { event } = evt.event;
    const state = await core.state();
    const v = state["toggle-state-debug"];
    const toggled = isTBool(v) ? v.bool : false;
    const init = state["state-viz-init"];
    switch (event) {
      case "toggle-state-debug":
        let state_builder = new StateBuilder().add("toggle-state-debug", tBool(!toggled));
        if (init === undefined) {
          state_builder = state_builder.add("state-viz-init", tBool(true));
        }
        updateTree(state);
        await core.sendModification((toggled
          ? new UiBuilder()
            .add_attr(cls("hide-debug-state"), "state-viz")
          : new UiBuilder()
            .rem_attr(cls("hide-debug-state"), "state-viz")).build(state_builder));
        return;
      default:
        return;
    }
  },
};

export default module;

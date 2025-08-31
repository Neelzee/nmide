import { click, HtmlBuilder, id, installModule, isPrimAnd, isTBool, isTInt, mkPrimEvent, StateBuilder, tInt, UiBuilder, type Core, type Event } from "@nmide/js-utils";

const name = "standalone-js-counter";

installModule({
  name,
  init: async (core: Core): Promise<void> => {
    await core.registerHandler(name, name);
  },
  handler: async (event: Event, core: Core): Promise<void> => {
    const state = await core.state();
    const disabled = !(isTBool(state[name]) && state[name].bool);
    if (disabled) return;
    await core.sendModification(
      new UiBuilder().add(
        new HtmlBuilder().kind("button")
          .attrs(
            id(name),
            click(mkPrimEvent(name))
          )
          .text("0")
      ).build()
    );
    if (isPrimAnd(event, name)) {
      const state = await core.state();
      const count = isTInt(state[name])
        ? state[name]
        : tInt(0);

      const newCount = tInt(count.int + 1);

      await core.sendModification(
        new StateBuilder()
          .add(name, newCount)
          .build(new UiBuilder().set_text(`${newCount.int}`, name))
      );
    }
  }
})

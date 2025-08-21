import {
  click,
  getArgs,
  HtmlBuilder,
  id,
  installModule,
  isPrimAnd,
  isTObj,
  mkPrimEvent,
  StateBuilder,
  tInt,
  tObj,
  tObjLookupOr,
  tValueMaybeOr,
  UiBuilder,
  type Core,
  type Event,
  type ValueInt,
  type ValueObj,
} from "@nmide/js-utils";

const name = "jsm-counter";

installModule({
  name,
  init: async (core: Core): Promise<void> => {
    await core.registerHandler(name, name);
    const ui = new UiBuilder().add(
      new HtmlBuilder().kids(
        new HtmlBuilder().kind("p")
          .text("0")
          .attrs(id(name)),
        new HtmlBuilder().kind("button")
          .text("Click")
          .attrs(click(mkPrimEvent(name, tInt(1))))
      )
    );
    await core.sendModification(ui.build(new StateBuilder().add(name, tInt(0))));
  },
  handler: async (event: Event, core: Core): Promise<void> => {
    if (isPrimAnd(event, name)) {
      const args = tValueMaybeOr<ValueObj>(getArgs(event))(tObj({}));
      console.log("args: ", args);
      const { int: increment } = tObjLookupOr<ValueInt>("eventArgs")(tInt(0))(args);
      console.log("tObjLookup: ", tObjLookupOr<ValueInt>("eventArgs")(tInt(0))(args));
      console.log("inrcr: ", increment);
      const { int: count } = tValueMaybeOr<ValueInt>((await core.state())[name])(tInt(0));
      const st = await core.state();
      console.log("State: ", st);
      console.log("State[name]: ", st[name]);
      console.log(count);
      const new_count = count + increment;
      const state = new StateBuilder().add(name, tInt(new_count));
      const ui = new UiBuilder().set_text(`${new_count}`, name);
      const modification = state.build(ui);
      await core.sendModification(modification);
    }
  }
});

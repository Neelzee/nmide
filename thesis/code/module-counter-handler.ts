const module: Module = {
  // ...
  handler: async (event: Event, core: Core): Promise<void> => {
    if (isPrimAnd(event, "count")) {
      const state = await core.state();
      const count = isTInt(state["count"])
        ? state[name]
        : tInt(0);

      const newCount = tInt(count.int + 1);

      await core.sendModification(
        new StateBuilder()
          .add("count", newCount)
          .build(
            new UiBuilder()
              .set_text(`${newCount.int}`, "counter-btn")
          )
      );
    }
  }
}

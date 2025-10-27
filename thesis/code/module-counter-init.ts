const module = {
  name: "conter-module",
  init: async (core: Core): Promise<void> => {
    await core.registerHandler("counter-module", "count");
    await core.sendModification(
      new Button()
        .attrs(
          id("counter-btn"),
          onClick(event("count"))
        )
        .text("0")
    );
  },
  // ...
}

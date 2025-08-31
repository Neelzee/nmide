const module = {
  // ...
  init: async (core: Core): Promise<void> => {
    await core.registerHandler("counter-module", "count");
    await core.sendModification(
      new UiBuilder().add(
        new HtmlBuilder().kind("button")
          .attrs(
            id("counter-btn"),
            click(mkPrimEvent("count"))
          )
          .text("0")
      ).build()
    );
  },
  // ...
}

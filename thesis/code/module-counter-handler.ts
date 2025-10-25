const module = {
  // ...
  handler: async (event: Event, core: Core): Promise<void> => {
    if (event == "count") {
      const state = await core.state();
      const count = state["count"];
      const newCount = count + 1;
      await core.sendModification(
        new Modification()
          .state("count", newCount)
          .ui(`${newCount}`, "counter-btn")
      );
    }
  }
}

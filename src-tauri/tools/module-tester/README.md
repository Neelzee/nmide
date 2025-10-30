# Module-tester

1. Load all bundles (JS, RS, etc...)
2. Await all `init` calls on each module
3. Record all event registrations
4. Initialize an empty event queue
5. For each module, record any events thrown during `init`
6. While event queue is not empty:
   1. Take an event from the queue
   2. For each `handler` registered for this event type:
      1. Await the handler
      2. Record any new events thrown during or after handler completion
      3. Add new events to queue
7. Once stable (no new events), analyze the recorded interactions


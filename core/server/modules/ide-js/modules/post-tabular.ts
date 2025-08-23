/**
 * Post-tabular, module for handling _PRE_ tabulation computation!
 *
 * The tabular family, required the framework module to be finished, before it
 * can start creating tabs, but other modules could, and should be able to
 * create tabs. This module ensures this, by being the first to register for the
 * `OPEN_NEW_TAB_EVENT`, and storing all the tabs in the state, until
 * `POST_FRAMEWORK_EVENT` is fired. Once this happens, it will register the
 * _main_ tabulation module for the `OPEN_NEW_TAB_EVENT`, and ignore all
 * remaining events.
 *
 */

import {
  cls,
  HtmlBuilder,
  isPrimAnd,
  isTBool,
  isTHtml,
  isTInt,
  isTObj,
  isTStr,
  mkPrimEvent,
  StateBuilder,
  tBool,
  tInt,
  tList,
  tObj,
  tStr,
  type Core,
  type Event,
  type HValueList,
  type Module,
  type ValueObj
} from "@nmide/js-utils";
import {
  DUMP_TABS_EVENT,
  OPEN_NEW_TAB_EVENT,
  POST_FRAMEWORK_EVENT
} from "../lib/constants";

const name = "post-tabular";
const PRE_FRAMEWORK_EVENT_STORAGE = `${name}-PRE_FRAMEWORK_EVENT_STORAGE`;
const INITIALIZED = `${name}-INITIALIZED`;

const PostTabular: Module = {
  name,
  init: async (core: Core): Promise<void> => {
    await core.registerHandler(name, POST_FRAMEWORK_EVENT);
    await core.registerHandler(name, OPEN_NEW_TAB_EVENT);
    await core.sendModification(
      new StateBuilder()
        .add(PRE_FRAMEWORK_EVENT_STORAGE, tList())
        .add(INITIALIZED, tBool(false))
        .build()
    )
  },
  handler: async (event: Event, core: Core): Promise<void> => {
    if (isPrimAnd(event, POST_FRAMEWORK_EVENT)) {
      await core.registerHandler("tabular", OPEN_NEW_TAB_EVENT);
      await core.sendModification(new StateBuilder().add(INITIALIZED, tBool(true)).build());
      const state = await core.state();
      const tabs = state[PRE_FRAMEWORK_EVENT_STORAGE];
      await core.eventThrower(mkPrimEvent(DUMP_TABS_EVENT, tabs));
      return;
    }
    if (isPrimAnd(event, OPEN_NEW_TAB_EVENT) && isTObj(event.event.args)) {
      const { obj } = event.event.args;
      const state = await core.state();
      const init = isTBool(state[INITIALIZED])
        ? state[INITIALIZED].bool
        : false;
      if (init) {
        return;
      }
      const tabs = state[PRE_FRAMEWORK_EVENT_STORAGE] as HValueList<ValueObj>;
      const id = tInt(tabs.list.length + 1);
      const title = isTStr(obj["title"])
        ? obj["title"]
        : tStr(`Tab-${id.int}`);
      const content = isTHtml(obj["content"])
        ? obj["content"]
        : undefined;
      if (content) {
        tabs.list.push(tObj({ title, content }));
      } else {
        tabs.list.push(tObj({ title }));
      }

      await core.sendModification(
        new StateBuilder().set(PRE_FRAMEWORK_EVENT_STORAGE, tabs).build()
      );
    }
  }
}

export default PostTabular;

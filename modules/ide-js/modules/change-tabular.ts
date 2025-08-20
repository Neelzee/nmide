/**
 * Simple module for changing tabs
 */

import {
  cls,
  HtmlBuilder,
  isPrimAnd,
  isTHtml,
  isTInt,
  isTObj,
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
  CHANGE_TAB_EVENT,
} from "../lib/constants";

const name = "change-tabular";

const CURRENT_TAB = `${name}-CURRENT_TAB`;
const TABS = `${name}-TABS`;

const ChangeTabular: Module = {
  name,
  init: async (core: Core): Promise<void> => {
    await core.registerHandler(name, CHANGE_TAB_EVENT);
    await core.sendModification(
      new StateBuilder()
        .add(CURRENT_TAB, "null")
        .add(TABS, tList())
        .build()
    )
  },
  handler: async (event: Event, core: Core): Promise<void> => {
    if (!isPrimAnd(event, CHANGE_TAB_EVENT) || !isTObj(event.event.args)) return;
    const state = await core.state();
  }
}

export default ChangeTabular;

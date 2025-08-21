import {
  click,
  cls,
  HtmlBuilder,
  id,
  isPrimAnd,
  isTHtml,
  isTInt,
  isTList,
  isTObj,
  mkPrimEvent,
  StateBuilder,
  tBool,
  tInt,
  tList,
  tObj,
  tStr,
  UiBuilder,
  type Core,
  type Event,
  type Html,
  type HValueList,
  type Module,
  type Value,
  type ValueList,
  type ValueObj
} from "@nmide/js-utils";
import {
  OPEN_NEW_TAB_EVENT,
  DUMP_TABS_EVENT,
} from "../lib/constants";
import { HIDE_TAB_HTML_CLASS, IDE_TAB_CONTAINER_HTML_CLASS, isTabContent, mkTabBtn, TAB_BTN_CONTAINER_HTML_CLASS, TAB_BTN_HTML_CLASS, type TabContentPair } from "../lib/tabularUtils";

const name = "tabular";

const CURRENT_TAB = `${name}-CURRENT_TAB`;
const TABS = `${name}-TABS`;

const Tabular: Module = {
  name,
  init: async (core: Core): Promise<void> => {
    await core.registerHandler(name, OPEN_NEW_TAB_EVENT);
    await core.sendModification(
      new StateBuilder()
        .add(CURRENT_TAB, "null")
        .add(TABS, tList())
        .build(
          new UiBuilder()
            .add(new HtmlBuilder().attrs(id(IDE_TAB_CONTAINER_HTML_CLASS)))
            .add(new HtmlBuilder().attrs(id(TAB_BTN_CONTAINER_HTML_CLASS)))
        )
    );
  },
  handler: async (event: Event, core: Core): Promise<void> => {
    if (isPrimAnd(event, OPEN_NEW_TAB_EVENT) && isTObj(event.event.args)) {
      return;
    }
    if (isPrimAnd(event, DUMP_TABS_EVENT) && isTList(event.event.args)) {
      const { list } = event.event.args;
      const tabs = list.filter(isTObj)
        .map(({ obj }) => isTabContent(obj) ? obj as TabContentPair : undefined)
        .filter(x => x !== undefined)
        .sort((a, b) => a.id - b.id);
      await Promise.all(tabs.map(t => newTab(core, t)));
      return;
    }
  }
}

const newTab = async (
  core: Core,
  {
    title,
    content,
  }: TabContentPair
): Promise<void> => {
  const state = await core.state();
  const cts: TabContentPair[] = (isTList(state[TABS])
    ? state[TABS].list
    : []).filter(isTObj)
    .map(({ obj }) => isTabContent(obj) ? obj as TabContentPair : undefined)
    .filter(x => x !== undefined)
    .sort((a, b) => a.id - b.id);
  const ct = cts[cts.length - 1] || { id: 0 };
  const ctId = isTInt(ct["id"])
    ? ct["id"].int
    : 0;
  const newId = ctId + 1;
  const htmlId = `tab-${newId}`;
  const newTitle = title || `Tab-${newId}`;
  const newContent = new HtmlBuilder().attrs(
    id(htmlId),
    cls(HIDE_TAB_HTML_CLASS)
  ).kids(content);
  const btn = mkTabBtn(newTitle, htmlId);
  await core.sendModification(
    new UiBuilder()
      .add(btn, TAB_BTN_CONTAINER_HTML_CLASS)
      .add(newContent, IDE_TAB_CONTAINER_HTML_CLASS)
      .build()
  )
}

export default Tabular;

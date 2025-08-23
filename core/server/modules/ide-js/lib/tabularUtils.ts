import { click, HtmlBuilder, id, mkPrimEvent, type Html } from "@nmide/js-utils"

export type Tab = {
  id: number,
  title?: string,
}

export type Content = {
  content: Html,
}

export type TabContentPair = Tab & Partial<Content>;

export const isTabContent = (x: unknown): x is TabContentPair =>
  typeof x === "object" &&
  x !== null &&
  "id" in x;

export const mkTabBtn = (title: string, _id: string) => new HtmlBuilder().kind("button")
  .text(title)
  .attrs(
    click(mkPrimEvent(`show-${_id}`)),
    id(TAB_BTN_HTML_CLASS)
  )

export const HIDE_TAB_HTML_CLASS = "hide-tab";
export const TAB_BTN_HTML_CLASS = "tab-btn";
export const TAB_BTN_CONTAINER_HTML_CLASS = "tab-btn-container";
export const IDE_TAB_CONTAINER_HTML_CLASS = "ide-tab-container"

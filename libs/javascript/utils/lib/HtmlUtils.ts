import type { Html } from "./Html";
import type { Attr } from "./Attr.ts";
import type { HtmlKind } from "./HtmlBuilder";
import { idCmp } from "./AttrUtils.ts";

export const getElementById = (
  id: string,
  ui: Html
): Html | undefined => attrs(ui).find(a => idCmp(a, id)) !== undefined
    ? ui
    : kids(ui).find(hasId(id))


export const hasId = (id: string) =>
  (ui: Html): boolean => attrs(ui).find(a => idCmp(a, id)) !== undefined

export const attrs = (ui: Html): Attr[] => {
  const key: HtmlKind = Object.keys(ui)[0];
  // @ts-expect-error This is valid
  return ui[key]["attrs"]
}

export const kids = (ui: Html): Html[] => {
  const key: HtmlKind = Object.keys(ui)[0];
  // @ts-expect-error This is valid
  return ui[key]["kids"]
}

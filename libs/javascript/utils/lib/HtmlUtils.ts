import type { Html } from "./Html";
import type { Attr } from "./Attr.ts";
import type { HtmlContent, HtmlKind } from "./HtmlBuilder";
import { attrsNanCheck, idCmp } from "./AttrUtils.ts";

export const getElementById = (
  id: string,
  ui: Html
): Html | undefined => {
  if (hasId(id)(ui)) {
    return ui;
  } else {
    return kids(ui)
      .map(k => getElementById(id, k))
      .filter(u => u !== undefined)
      .find(() => true);
  }
}


export const applyById = (id: string) => (f: (a: Html) => Html) => (ui: Html): Html => {
  if (hasId(id)(ui)) {
    return f(ui);
  } else {
    return replaceKids(
      ui,
      kids(ui).map(applyById(id)(f))
    );
  }
}

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

export const replaceKids = (ui: Html, kids: Html[]): Html => {
  const key: HtmlKind = Object.keys(ui)[0];
  // @ts-expect-error This is valid
  ui[key]["kids"] = kids;
  return ui;
}

export const allAttrs = (ui: Html): Attr[] => {
  const key: HtmlKind = Object.keys(ui)[0];
  // @ts-expect-error This is valid
  const attrs = ui[key]["attrs"];
  // @ts-expect-error This is valid
  return [...attrs, ...ui[key]["kids"].flatMap(allAttrs)]
}

export const htmlNanCheck = (ui: Html): Html => {
  const key: HtmlKind = Object.keys(ui)[0];
  // @ts-expect-error This is valid
  const { kids, attrs, text }: HtmlContent = ui[key];

  // @ts-expect-error This is valid
  ui[key] = { kids: kids.map(htmlNanCheck), attrs: attrs.map(attrsNanCheck), text };
  return ui;
}

export const addKid = (ui: Html, kid: Html): Html  => {
  const key: HtmlKind = Object.keys(ui)[0];
  
  // @ts-expect-error This is valid
  ui[key]["kids"].push(kid);

  return ui;
}

export const addAttr = (ui: Html, attr: Attr): Html => {
  const key: HtmlKind = Object.keys(ui)[0];

  // @ts-expect-error This is valid
  ui[key]["attr"].push(attr);

  return ui;
}

export const remAttr = (ui: Html, attr: Attr): Html => {
  const key: HtmlKind = Object.keys(ui)[0];
  const attr_key = Object.keys(attr)[0];

  // @ts-expect-error This is valid
  ui[key]["attr"].filter((a: Attr) => !(attr_key in a));

  return ui;
}

export const setText = (ui: Html, s: string): Html  => {
  const key: HtmlKind = Object.keys(ui)[0];
  
  // @ts-expect-error This is valid
  ui[key]["text"] = s;

  return ui;
}
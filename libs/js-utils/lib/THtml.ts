import { TAttr } from "./TAttr";

type THtmlKind = "div" | "p" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | "span"
  | "section" | "article" | "aside" | "audio" | "b" | "br" | "button" | "code"
  | "em" | "fieldset" | "form" | "img" | "input" | "label" | "link" | "li"
  | "menu" | "nav" | "ol" | "option" | "select" | "style" | "svg" | "table"
  | "td" | "th" | "ul" | "video" | "frag" | "text" | "script" | "tr" | "tbody"
  | "main"

export type THtml = {
  kind: THtmlKind;
  kids: THtml[];
  attrs: TAttr[];
  text: string | undefined;
};

import { type Html } from "./Html";
import { type Attr } from "./Attr";

export type HtmlKind = Html extends { [K: string]: any } ? keyof Html : never;
export type HtmlContent = Html extends { [K: string]: infer T } ? T : never;
export class HtmlBuilder {
  private _kind: HtmlKind;
  private _kids: Array<Html | HtmlBuilder>
  private _attrs: Array<Attr>;
  private _text: string | null;

  constructor() {
    this._kind = "div";
    this._kids = [];
    this._attrs = [];
    this._text = null;
  }

  kind(kind: HtmlKind): HtmlBuilder {
    this._kind = kind;
    return this;
  }

  kids(...kids: (Html | HtmlBuilder)[]): HtmlBuilder {
    this._kids.push(...kids);
    return this;
  }

  text(text: string): HtmlBuilder {
    this._text = text;
    return this;
  }

  attrs(...attrs: (undefined | Attr)[]): HtmlBuilder {
    this._attrs.push(...(attrs.filter(a => a !== undefined)));
    return this;
  }

  build(): Html {
    const obj = {};
    // @ts-ignore keyof Html is a string
    obj[this._kind] = {
      attrs: this._attrs,
      kids: this._kids.map(h => {
        if (h instanceof HtmlBuilder) {
          return h.build();
        } else {
          return h;
        }
      }),
      text: this._text
    }
    return obj as Html;
  }
}

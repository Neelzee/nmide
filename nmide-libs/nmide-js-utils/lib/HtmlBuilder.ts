import { TAttr } from "./TAttr";
import { THtml } from "./THtml";
import { THtmlKind } from "./THtmlKind";

export default class HtmlBuilder {
  _kind: THtmlKind = "Frag";
  _kids: (THtml | HtmlBuilder)[] = [];
  _attrs: TAttr[] = [];
  _text: null | string = null;

  kind(kind: THtmlKind): HtmlBuilder {
    this._kind = kind;
    return this;
  }

  kids(kids: (THtml | HtmlBuilder)[]): HtmlBuilder {
    this._kids = kids;
    return this;
  }

  attrs(attrs: TAttr[]): HtmlBuilder {
    this._attrs = attrs;
    return this;
  }

  text(text: string): HtmlBuilder {
    this._text = text;
    return this;
  }

  build(): THtml {
    return {
      kind: this._kind,
      kids: this._kids.map(v => {
        if (v instanceof HtmlBuilder) {
          return v.build();
        } else {
          return v;
        }
      }),
      attrs: this._attrs,
      text: this._text,
    };
  }
}

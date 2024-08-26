import { Fragment } from "react/jsx-runtime";
import { Html } from "../bindings/Html";
import { v4 as uuidv4 } from "uuid";
import { Attr } from "src/bindings/Attr";
import { Msg } from "src/bindings/Msg";
import TauriClient from "../client";
import { Css } from "src/bindings/Css";

export default function RenderHtml({ html }: { html: Html }) {
  if (typeof html !== "object") {
    return <></>
  }
  if ("Div" in html) {
    const attrs = ToAttrObj(html.Div);
    return (
      <div
        key={uuidv4()}
        className={attrs["Class"] === undefined ? undefined : `${attrs["Class"]}`}
        id={attrs["Id"] === undefined ? undefined : `${attrs["Id"]}`}
        onClick={() => {
          const msg = attrs["OnClick"];
          if (msg === "undefined" || typeof msg === "string") {
            return;
          }
          TauriClient("process_msg", { msg: msg as Msg }).catch(err => console.error(err));
        }}
      >
        {html.Div.kids.map(k => {
          return <RenderHtml html={k} />
        }
        )}
      </div>
    );
  } else if ("Button" in html) {
    const attrs = ToAttrObj(html.Button);
    return (
      <button
        key={uuidv4()}
        className={attrs["Class"] === undefined ? undefined : `${attrs["Class"]}`}
        id={attrs["Id"] === undefined ? undefined : `${attrs["Id"]}`}
        onClick={() => {
          const msg = attrs["OnClick"];
          if (msg === "undefined" || typeof msg === "string") {
            return;
          }
          TauriClient("process_msg", { msg: msg as Msg }).catch(err => console.error(err));
        }}
      >
        {html.Button.kids.map(k => {
          return <RenderHtml html={k} />
        }
        )}
      </button>
    );
  } else if ("P" in html) {
    const attrs = ToAttrObj(html.P);
    return (
      <p
        key={uuidv4()}
        className={attrs["Class"] === undefined ? undefined : `${attrs["Class"]}`}
        id={attrs["Id"] === undefined ? undefined : `${attrs["Id"]}`}
        onClick={() => {
          const msg = attrs["OnClick"];
          if (msg === "undefined" || typeof msg === "string") {
            return;
          }
          TauriClient("process_msg", { msg: msg as Msg }).catch(err => console.error(err));
        }}
      >
        {html.P.kids.map(k => {
          return <RenderHtml html={k} />
        }
        )}
      </p>
    );
  }

  if ("Text" in html) {
    return <Fragment key={uuidv4()}>{html["Text"]}</Fragment>;
  }

  return <Fragment key={uuidv4()} />;
}

type ExtractKeys<T> = T extends { [K in keyof T]: any } ? keyof T : never;
type AttrKeys = ExtractKeys<Attr>;
type AttrObj = Record<AttrKeys | string, string | Css[] | Msg>;

export function ToAttrObj({ attrs }: { attrs: Array<Attr> }): AttrObj {
  const map: AttrObj = {};
  attrs.forEach(attr => {
    if ("Id" in attr) {
      map["Id"] = attr.Id;
    } else if ("Class" in attr) {
      map["Class"] = attr.Class;
    } else if ("Src" in attr) {
      map["Src"] = attr.Src;
    } else if ("Alt" in attr) {
      map["Alt"] = attr.Alt;
    } else if ("OnClick" in attr) {
      map["OnClick"] = attr.OnClick;
    } else if ("For" in attr) {
      map["For"] = attr.For;
    } else if ("Style" in attr) {
      map["Style"] = attr.Style;
    } else {
      map[attr.Attr[0]] = attr.Attr[1];
    }
  });
  return map;
}

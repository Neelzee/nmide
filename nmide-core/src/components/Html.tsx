import { Fragment } from "react/jsx-runtime";
import { Html } from "../bindings/Html";
import { v4 as uuidv4 } from "uuid";
import { Attr } from "src/bindings/Attr";
import { Msg } from "src/bindings/Msg";
import { emit } from "@tauri-apps/api/event";

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
          emit(`${msg}`, {}).catch(err => console.error(err));
        }}
      >
        {html.Div.kids.map(k => {
          return <RenderHtml html={k} />
        }
        )}
      </div>
    );

  }

  return <Fragment key={uuidv4()}>{html.Text}</Fragment>
}

type ExtractKeys<T> = T extends { [K in keyof T]: any } ? keyof T : never;
type AttrKeys = ExtractKeys<Attr>;

export function ToAttrObj({ attrs }: { attrs: Array<Attr> }): Record<AttrKeys | string, string | Msg | undefined> {
  const map: Record<AttrKeys | string, string | Msg> = {};
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
    } else {
      map[attr.Attr[0]] = attr.Attr[1];
    }
  });
  return map;
}

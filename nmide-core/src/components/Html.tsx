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
        className={attrs.Class}
        id={attrs.Id}
        style={RsStyleToReactCSSProperties(attrs.Style)}
        onClick={OnClickParse(attrs.OnClick)}
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
        className={attrs.Class}
        id={attrs.Id}
        onClick={OnClickParse(attrs.OnClick)}
        style={RsStyleToReactCSSProperties(attrs.Style)}
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
        className={attrs.Class}
        id={attrs.Id}
        onClick={OnClickParse(attrs.OnClick)}
        style={RsStyleToReactCSSProperties(attrs.Style)}
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

function OnClickParse(msg: Msg | undefined): () => void {
  return () => {
    if (msg === undefined) {
      return;
    }
    TauriClient("process_msg", { msg: msg as Msg }).catch(err => console.error(err));
  };
}

type StdAttr = {
  OnClick?: Msg,
  Style?: Css[],
  For?: string,
  Id?: string,
  Class?: string,
  Src?: string,
  Alt?: string,
};

type CstmAttr = {
  [key: string]: string | undefined,
};

type AttrObj = StdAttr & CstmAttr;

export function ToAttrObj({ attrs }: { attrs: Array<Attr> }): AttrObj {
  const map: AttrObj = {};
  attrs.forEach(attr => {
    if ("Id" in attr) {
      map.Id = attr.Id;
    } else if ("Class" in attr) {
      map.Class = attr.Class;
    } else if ("Src" in attr) {
      map.Src = attr.Src;
    } else if ("Alt" in attr) {
      map.Alt = attr.Alt;
    } else if ("OnClick" in attr) {
      map.OnClick = attr.OnClick;
    } else if ("For" in attr) {
      map.For = attr.For;
    } else if ("Style" in attr) {
      map.Style = attr.Style;
    } else {
      const [key, val] = attr.Attr;
      map[key] = val;
    }
  });
  return map;
}

function RsStyleToReactCSSProperties(css: Css[] | undefined): React.CSSProperties {
  if (css === undefined) {
    return {};
  }
  const props: React.CSSProperties = {};

  css.forEach(({ styles }) => {
    styles.forEach(([_, style]) => {
      if ("Width" in style) {
        props.width = `${style.Width[0]}${style.Width[1]}`.toLowerCase();
      } else if ("PaddingLeft" in style) {
        props.paddingLeft = `${style.PaddingLeft[0]}${style.PaddingLeft[1]}`.toLowerCase();
      } else if ("BackgroundColor" in style) {
        props.backgroundColor = `rgb(${style.BackgroundColor.r} ${style.BackgroundColor.g} ${style.BackgroundColor.b})`;
      }
    });
  });

  return props;
}

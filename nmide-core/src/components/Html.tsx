import { Fragment } from "react/jsx-runtime";
import { Html } from "../bindings/Html";
import { v4 as uuidv4 } from "uuid";

export default function RenderHtml(props: { html: Html }) {
  const html = props.html;
  switch (html.kind) {
    case "P":
      return <p key={uuidv4()}>{html.kids.map(k => {
        return <RenderHtml html={k} />
      })}</p>

    case "Div":
      return <div key={uuidv4()}>{html.kids.map(k => {
        return <RenderHtml html={k} />
      })}</div>

    default:
      if (typeof html.kind === "object" && "Text" in html.kind) {
        return <Fragment key={uuidv4()}>{html.kind.Text}</Fragment>
      }
      return (
        <>{html.kids.map((k) => {
          return <RenderHtml html={k} />
        })}</>
      );
  }
}

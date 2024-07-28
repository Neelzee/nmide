import { Html } from "../bindings/Html";

export default function RenderHtml(props: { html: Html }) {
  const html = props.html;
  switch (html.kind) {
    case "P":
      return <p>{html.kids.map(k => {
        return <RenderHtml html={k} />
      })}</p>

    case "Div":
      return <div>{html.kids.map(k => {
        return <RenderHtml html={k} />
      })}</div>

    default:
      return <>{html.kids.map(k => {
        return <RenderHtml html={k} />
      })}</>
  }
}

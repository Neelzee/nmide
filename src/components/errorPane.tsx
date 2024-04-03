import { Accessor, createEffect, createSignal } from "solid-js";
import "../styles/error_pane.scss";
import { NmideReport } from "../types";

export default function ErrorPane(props: { errors: Accessor<NmideReport[]> }) {
  const [err, setErr] = createSignal<NmideReport[]>([]);
  createEffect(() => {
    setErr(props.errors());
  })
  return (
    <section class="error-pane">
      {
        err().map(e => <RenderError error={e} />)
      }
    </section>
  );
}


function RenderError(props: { error: NmideReport }) {
  const error = props.error;
  return (
    <section class="error-msg {error.lvl} {error.tag}">
      <span>Error: {error.msg}</span>
      <span>Level: {error.lvl}</span>
      <span>Tag: {error.tag}</span>
      <span>Origin: {error.origin}</span>
      <span>Stacktrace: {error.stack.map(e => RenderError(e))}</span>
    </section>
  );
}

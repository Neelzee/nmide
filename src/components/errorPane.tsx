import { createSignal } from "solid-js";
import "../styles/error_pane.scss";
import { NmideReport } from "../types";

export function ErrorPane(props: { errors: NmideReport[] }) {
  const [errors, setErrors] = createSignal<NmideReport[]>([]);

  setErrors(props.errors);

  return (
    <section class="error-pane">
      {
        errors().map(e => <RenderError error={e} />)
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

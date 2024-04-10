
import { Accessor, createEffect, createSignal } from "solid-js";
import "@styles/error_pane.scss";
import { NmideReport } from "../types";

export default function ErrorPane(props: { errors: Accessor<NmideReport[]> }) {
  const [err, setErr] = createSignal<NmideReport[]>([]);

  createEffect(() => {
    console.log("errorpane");
    // Synchronize local state with props
    setErr(props.errors());
    // Cleanup effect to avoid memory leaks
    return () => { };
  });

  return (
    <section class="error-pane">
      {err().map((e, index) => (
        <RenderError key={`${index}`} error={e} />
      ))}
    </section>
  );
}

function RenderError(props: { error: NmideReport, key: string }) {
  const error = props.error;
  return (
    <section class={`error-msg ${error.lvl} ${error.tag}`} id={props.key}>
      <span>Error: {error.msg}</span>
      <span>Level: {error.lvl}</span>
      <span>Tag: {error.tag}</span>
      <span>Origin: {error.origin}</span>
      <span>Stacktrace: {error.stack.map((e, index) => (
        <span id={`${index}`}>{e.msg}</span>
      ))}</span>
    </section>
  );
}


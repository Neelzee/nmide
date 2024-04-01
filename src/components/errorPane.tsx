import { createSignal } from "solid-js";
import "../styles/error_pane.scss";

export function ErrorPane(props: { errors: string[] }) {
  const [errors, setErrors] = createSignal<string[]>([]);

  setErrors(props.errors);

  return (
    <>
      <section class="error-pane">
        {
          errors().map(e => <p>{e}</p>)
        }
      </section>
    </>
  );
}

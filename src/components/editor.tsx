import { createSignal, JSX, For, Accessor, createEffect } from "solid-js";

export default function Editor(props: { content: Accessor<string[]> }): JSX.Element {
  const [content, setContent] = createSignal<string[]>([]);

  createEffect(() => setContent(props.content()));

  return (
    <section>
      <For each={content()}>
        {(line, index) => <p id={`${index}`}>{line}</p>}
      </For>
    </section>
  );
}

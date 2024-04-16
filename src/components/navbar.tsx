import { JSX, createSignal, For, Show, Accessor, createEffect } from "solid-js";

export default function NavBar(props: { pages: Accessor<number[]> }): JSX.Element {
  const [pages, setPages] = createSignal<number[]>([]);
  createEffect(() => {
    setPages(props.pages());
  });
  return (
    <section>
      <Show when={pages().length !== 0} fallback={EmptyBar()}>
        <For each={pages()}>
          {(item, index) =>
            <div id={`${index()}`}>{item}</div>
          }
        </For>
      </Show>
    </section >
  );
}

function EmptyBar(): JSX.Element { return <div>+</div> }

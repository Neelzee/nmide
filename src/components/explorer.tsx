import { Accessor, createEffect, createSignal } from "solid-js";
import { Folder, File } from "../types.ts";

export default function Explorer(props: { files: Accessor<Folder> }) {
  const [folder, setFolder] = createSignal<Folder>({ name: "", path: "", content: [] });

  createEffect(() => {
    setFolder(props.files());
  })

  return (
    <section class="explorer">
      <RenderFolder folder={folder()} />
    </section>
  );
}

function RenderFile(props: { file: File }) {
  const file = props.file;
  return (
    <span class={`file ${file.extension} ${file.name}`} >
      <span>{file.name}</span>
    </span >
  );
}

function RenderFolder(props: { folder: Folder }) {
  const folder = props.folder;
  return (
    <span>
      <span>{folder.name}</span>
      <span>
        {
          folder.content.map(fof => {
            if ("content" in fof) {
              const sf = fof as Folder;
              return <RenderFolder folder={sf} />
            } else {
              const sf = fof as File;
              return <RenderFile file={sf} />
            }
          })
        }
      </span>
    </span>
  );
}

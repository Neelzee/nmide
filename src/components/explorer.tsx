
import { Accessor, createEffect, createSignal } from "solid-js";
import { Folder, File } from "../types";

export default function Explorer(props: { files: Accessor<Folder> }) {
  const [folder, setFolder] = createSignal<Folder>({ name: "", path: "", content: [] });

  createEffect(() => {
    console.log("explorer");
    // Synchronize local state with props
    setFolder(props.files());
    // Cleanup effect to avoid memory leaks
    return () => { };
  });

  const f = folder();

  return (
    <section class="explorer">
      <RenderFolder key={f.path} folder={f} />
    </section>
  );
}

function RenderFile(props: { file: File, key: string }) {
  const file = props.file;
  return (
    <div class={`file ${file.extension} ${file.name}`} >
      <span>{file.name}</span>
    </div>
  );
}

function RenderFolder(props: { folder: Folder, key: string }) {
  const folder = props.folder;
  return (
    <ul id={props.key}>
      <li>{folder.name}</li>
      <ul>
        {folder.content.map(fof => {
          if ("content" in fof) {
            const sf = fof as Folder;
            return <RenderFolder key={sf.path} folder={sf} />;
          } else {
            const sf = fof as File;
            return <RenderFile key={sf.path} file={sf} />;
          }
        })}
      </ul>
    </ul>
  );
}


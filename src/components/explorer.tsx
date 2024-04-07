import { Accessor, createEffect, createSignal } from "solid-js";
import { Folder, File } from "../types";

export default function Explorer(props: { files: Accessor<Folder> }) {
  const [folder, setFolder] = createSignal<Folder>({ name: "", path: "", content: [] });

  createEffect(() => {
    console.log(props.files());
    // Synchronize local state with props
    setFolder(props.files());
  });

  const f = folder();
  console.log("Rerender?");

  return (
    <section class="explorer">
      <RenderFolder key={f.path} folder={folder()} />
    </section>
  );
}

function RenderFile(props: { file: File, key: string }) {
  const [file, setFile] = createSignal<File>({} as File);

  createEffect(() => {
    setFile(props.file);
  });

  return (
    <span class={`file ${file().extension} ${file().name}`}>
      {file().name}
    </span>
  );
}

function RenderFolder(props: { folder: Folder, key: string }) {
  const [folder, setFolder] = createSignal<Folder>(props.folder);

  createEffect(() => {
    setFolder(props.folder);
  });

  return (
    <ul id={props.key}>
      <li>{folder().name}</li>
      <ul>
        {folder().content.map(fof => {
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


import { Accessor, createEffect, createSignal } from "solid-js";
import { Folder, File } from "../types";

export default function Explorer(props: { files: Accessor<Folder> }) {
  const [folder, setFolder] = createSignal<Folder>({ name: "", path: "", content: [] });

  createEffect(() => {
    // Synchronize local state with props
    setFolder(props.files());
  });

  const f = folder();

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
    <span class={`file ${file().extension} ${file().name}`} >
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
        {folder().content.map(f => {
          if ("content" in f) {
            const sf = f as Folder;
            return <RenderFolder key={sf.path} folder={sf} />;
          } else {
            const sf = f as File;
            return <RenderFile key={sf.path} file={sf} />;
          }
        })}
      </ul>
    </ul>
  );
}




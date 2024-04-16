import { JSX, Accessor, Show, createEffect, createSignal } from "solid-js";
import { Folder, File } from "../types";
import "@styles/explorer.scss";

export default function Explorer(props: { files: Accessor<Folder> }) {
  const [folder, setFolder] = createSignal<Folder>({ name: "", path: "", content: [] });

  createEffect(() => {
    // Synchronize local state with props
    setFolder(props.files());
  });

  const f = folder();

  return (
    <section class="explorer">
      <Show when={folder().name !== ""} fallback={EmptyExplorer()}>
        <RenderFolder key={f.path} folder={folder()} />
      </Show>
    </section>
  );
}

function RenderFile(props: { file: File, key: string }) {
  const [file, setFile] = createSignal<File>({} as File);

  createEffect(() => {
    setFile(props.file);
  });

  const fileName = () => {
    console.log(file().name);
  };

  return (
    <span
      class={`file ${file().extension} ${file().name}`}
      onClick={fileName}
    >
      {file().name}
    </span>
  );
}

function RenderFolder(props: { folder: Folder, key: string }) {
  const [folder, setFolder] = createSignal<Folder>(props.folder);

  createEffect(() => {
    setFolder(props.folder);
  });

  const folderName = () => {
    console.log(folder().name);
  }

  return (
    <ul id={props.key} class="folder">
      <li
        class={`folder-name ${folder().name}`}
        onClick={folderName}
      >
        {folder().name}
      </li>
      <ul class="folder-content">
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
    </ul >
  );
}


function EmptyExplorer(): JSX.Element { return <div>Open project</div> }

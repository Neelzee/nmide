import { JSX, Accessor, Show, createEffect, createSignal, Setter } from "solid-js";
import { Folder, File } from "../types";
import "@styles/explorer.scss";
import { invoke } from "@tauri-apps/api";

export type ExplorerProps = {
  files: Accessor<Folder>,
  content: Accessor<string[]>,
  curPage: Setter<(props: Accessor<string[]>) => JSX.Element>
  loading: Accessor<boolean>,
};

export default function Explorer(props: ExplorerProps) {
  const [folder, setFolder] = createSignal<Folder>({ name: "", path: "", content: [], symbol: "" });
  const [loading, setLoading] = createSignal(false);

  createEffect(() => {
    // Synchronize local state with props
    setFolder(props.files());
  });

  createEffect(() => {
    setLoading(props.loading());
  });

  const f = folder();

  return (
    <section class="explorer">
      <Show when={folder().name !== ""} fallback={EmptyExplorer()}>
        <Show when={!loading()} fallback={<h3>Loading...</h3>}>
          <RenderFolder key={f.path} folder={folder()} />
        </Show>
      </Show>
    </section>
  );
}

function RenderFile(props: { file: File, key: string }) {
  const [file, setFile] = createSignal<File>({} as File);

  createEffect(() => {
    setFile(props.file);
  });

  const openFile = () => {
    invoke("get_content", { path: file() })
      .then(res => console.log(res))
      .catch(err => console.error(err));
  }

  return (
    <li
      class={`file ${file().extension} ${file().name}`}
      onClick={openFile}
    >
      {`${file().symbol}${file().name}`}
    </li>
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
        {`${folder().symbol}${folder().name}`}
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

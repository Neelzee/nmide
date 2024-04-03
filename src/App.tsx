import ToolBar from "./components/toolbar";
import Explorer from "./components/explorer";
import "./styles/main.scss";
import { invoke } from "@tauri-apps/api";
import { attachConsole } from "tauri-plugin-log-api";
import ErrorPane from "./components/errorPane";
import { createEffect, createSignal } from "solid-js";
import { NmideReport, NmideError, FolderOrFile, Folder, File } from "./types";
import { split_with_err } from "./funcs";
import { produce } from "solid-js/store";

const detach = await attachConsole();
// detach the browser console from the log stream
detach();

function App() {
  const [errors, setErrors] = createSignal<NmideReport[]>([]);
  const [folders, setFolders] = createSignal<Folder>({ name: "", path: "", content: [] });
  const [root, setRoot] = createSignal("");

  createEffect(() => {
    console.log(root());
    if (root() !== "") {
      invoke<NmideError<FolderOrFile>>("get_workspace", { path: root() })
        .then(res => {
          console.log("START")
          console.log(res);
          const [val, rep] = split_with_err<FolderOrFile>(res);
          if (rep !== null) {
            setErrors(produce(arr => {
              arr.push(rep);
            }));
          }
          if ("content" in val) {
            setFolders(val);
          } else {
            // Its a file
            setFolders({
              name: "nmide",
              path: "nmide",
              content: [val]
            });
          }
        })
        .catch(err => {
          setErrors(produce(arr => arr.push(err)));
        });
    }
  });
  return (
    <main>
      <ToolBar setRoot={setRoot} />
      <article>
        <ul id={root()}>
          <li>{folders().name}</li>
          <ul>
            {folders().content.map(fof => {
              console.log(fof);
              console.log((fof as Folder).content !== undefined);
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
        <ErrorPane errors={errors} />
      </article>
    </main>
  );
}

function RenderFile(props: { file: File, key: string }) {
  const file = props.file;
  return (
    <ul class={`file ${file.extension} ${file.name}`} >
      <li>{file.name}</li>
    </ul>
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


export default App;

import ToolBar from "@components/toolbar";
import Explorer from "@components/explorer";
import "@styles/main.scss";
import { invoke } from "@tauri-apps/api";
import ErrorPane from "@components/errorPane";
import { createEffect, createSignal, JSX, Accessor } from "solid-js";
import { NmideReport, NmideError, FolderOrFile, Folder } from "./types";
import { split_with_err } from "./funcs";
import { produce } from "solid-js/store";
import { Dynamic } from "solid-js/web";


function App() {
  const [errors, setErrors] = createSignal<NmideReport[]>([]);
  const [folders, setFolders] = createSignal<Folder>({ name: "", path: "", content: [], symbol: "" });
  const [root, setRoot] = createSignal("");
  const [pages, setPages] = createSignal<((props: any) => JSX.Element)[]>([]);

  const explorer = (props: { files: Accessor<Folder> }) => Explorer(props);
  const errorPane = (props: { errors: Accessor<NmideReport[]> }) => ErrorPane(props);

  setPages([explorer, errorPane]);

  createEffect(() => {
    if (root() !== "") {
      invoke<NmideError<FolderOrFile>>("get_workspace", { path: root() })
        .then(res => {
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
              name: val.name,
              path: val.path,
              content: [val],
              symbol: val.symbol,
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
        <Dynamic component={pages()[0]} files={folders} errors={errors} />
      </article>
    </main>
  );
}

export default App;

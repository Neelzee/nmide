import { SetStateAction, useState } from "react";
import { EditorWindow } from "./components/editor_window";
import { MenuBar } from "./components/menubar";
import "./styles/main.scss";
import { ExplorerPane } from "./components/explorer-pane/ExplorerPane";
import { Folder, FolderOrFile, Workspace, File } from "./types/types";
import React from "react";

export const WorkspaceContext = React.createContext<{
  root: String;
  open_files: File[];
  contents: FolderOrFile[];
  setWorkspace: (_: any) => void;
}>({
  root: ".",
  open_files: [],
  contents: [],
  setWorkspace: () => {},
});

function App() {
  const [workspace, setWorkspace] = useState<Workspace>({
    root: ".",
    open_files: [],
    contents: [],
  });
  return (
    //@ts-ignore
    <WorkspaceContext.Provider
      value={{ ...workspace, setWorkspace: setWorkspace }}
    >
      <MenuBar root={workspace.root} />
      <main>
        <ExplorerPane root={workspace.root} open_files={workspace.open_files} />
        <EditorWindow />
      </main>
    </WorkspaceContext.Provider>
  );
}

export default App;

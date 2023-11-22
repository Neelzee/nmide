import React from "react";
import { invoke } from "@tauri-apps/api";
import { File, Folder, FolderOrFile } from "../types/types";

interface ExplorerState {
  files: Folder;
  isLoading: Boolean;
}

export class Explorer extends React.Component {
  //@ts-ignore
  constructor(props) {
    super(props);
    this.state = { files: null, isLoading: true };
  }

  componentDidMount(): void {
    invoke("get_content_in_folder", {
      root: "C:\\Users\\nilsi\\Documents\\nario\\src",
    })
      .then((res) => {
        //@ts-ignore
        this.setState({ files: res, isLoading: false });
      })
      .catch((err) => console.error(err));
  }

  render() {
    //@ts-ignore
    const { isLoading, files } = this.state;

    //@ts-ignore
    if (isLoading) {
      return <div>Loading...</div>;
    }

    console.log(files);

    //@ts-ignore
    return (
      <section className="explorer">
        {/* @ts-ignore */}
        {
          //<RenderFolderOrFile folder={files} />
        }
      </section>
    );
  }
}

function RenderFolderOrFile(props: { folder: any | FolderOrFile }) {
  // @ts-ignore
  if (props.folder.File !== undefined) {
    // @ts-ignore
    return RenderFile(props.folder.File);
  }

  return (
    <div>
      {props.folder.Folder.name}
      {/* @ts-ignore */}
      {props.folder.Folder.contents.map((r) => {
        //@ts-ignore
        return RenderFolderOrFile(r);
      })}
    </div>
  );
}

function RenderFile(props: { file: File }) {
  return <div>{props.file.name}</div>;
}

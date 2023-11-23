import React, { useState } from "react";
import { invoke } from "@tauri-apps/api";
import { File, Folder, FolderOrFile } from "../types/types";
import "../styles/explorer.scss";
import { get_content_from_folder } from "./backend_api";

export class Explorer extends React.Component {
  //@ts-ignore
  constructor(props) {
    super(props);
    this.state = { files: null, isLoading: true };
  }

  componentDidMount(): void {
    let root = "C:\\Users\\nilsi\\Documents\\nmide";

    get_content_from_folder(root)
      .then((res) => {
        this.setState({ files: res, isLoading: false });
      })
      .catch((err) => {
        this.setState({ files: null, isLoading: true });
        console.error(err);
      });
  }

  render() {
    //@ts-ignore
    const { isLoading, files } = this.state;

    //@ts-ignore
    if (isLoading) {
      return <div>Loading...</div>;
    }

    //@ts-ignore
    return (
      <section className="explorer">
        {/* @ts-ignore */}
        <FileSystemRenderer data={files} />
      </section>
    );
  }
}

type Props = {
  data: Folder;
};

const FileSystemRenderer: React.FC<Props> = ({ data }) => {
  const [collapsedFolders, setCollapsedFolders] = useState<Set<string>>(
    new Set()
  );

  const toggleFolder = (folderPath: string) => {
    setCollapsedFolders((prevState) => {
      const newState = new Set(prevState);
      if (prevState.has(folderPath)) {
        newState.delete(folderPath);
      } else {
        newState.add(folderPath);
      }
      return newState;
    });
  };

  const renderFolderOrFile = (item: FolderOrFile, depth: number = 0) => {
    if ("contents" in item) {
      //@ts-ignore
      let folder: Folder = item;
      return (
        <div
          key={folder.path}
          className={`folder ${folder.name} ${
            collapsedFolders.has(folder.path) ? "collapsed" : ""
          }${depth === 0 ? "top" : ""}`}
        >
          <span
            onClick={() => toggleFolder(folder.path)}
            className="folder-name"
          >
            {`📂${folder.name}`}
          </span>
          {!collapsedFolders.has(folder.path) &&
            folder.contents.map((subItem) =>
              renderFolderOrFile(subItem, depth + 1)
            )}
        </div>
      );
    } else {
      //@ts-ignore
      let file: File = item;
      return (
        <div key={file.path} className={`file ${file.name}`}>
          <span className="file-name">{`📄${file.name}`}</span>
        </div>
      );
    }
  };

  return <div>{renderFolderOrFile(data)}</div>;
};

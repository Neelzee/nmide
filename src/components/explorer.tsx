import React, { useState } from "react";
import { invoke } from "@tauri-apps/api";
import { File, Folder, FolderOrFile } from "../types/types";
import "../styles/explorer.scss";

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

    console.log(files);

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
    if ("Folder" in item) {
      //@ts-ignore
      let folder: Folder = item.Folder;
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
    } else if ("File" in item) {
      //@ts-ignore
      let file: File = item.File;
      return (
        <div key={file.path} className={`file ${file.name}`}>
          <span className="file-name">{`📄${file.name}`}</span>
        </div>
      );
    }
  };

  return <div>{renderFolderOrFile(data)}</div>;
};

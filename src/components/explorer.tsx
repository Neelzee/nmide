import React, { useState } from "react";
import { invoke } from "@tauri-apps/api";
import { File, Folder, FolderOrFile } from "../types/types";
import "../styles/explorer.scss";
import { get_content_from_folder, open_file } from "./backend_api";
import { execSync } from "child_process";

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
        //@ts-ignore
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
        <FolderComponent
          name={files.name}
          path={files.path}
          icon={"📂"}
          contents={files.contents}
          className={"top"}
        />
      </section>
    );
  }
}

type FileProps = {
  name: string;
  path: string;
  icon: "📄";
};

class FileComponent extends React.Component<FileProps> {
  render() {
    const { name, path, icon } = this.props;
    //@ts-ignore
    const openFile = () => {
      open_file(`${path}\\${name}`)
        .then((res) => console.log(res))
        .catch((err) => console.error(err));
    };

    return (
      <div key={path} className={`file ${name}`}>
        <span className="file-name" onClick={openFile}>{`${icon}${name}`}</span>
      </div>
    );
  }
}

type FolderProps = {
  name: string;
  path: string;
  icon: "📂";
  contents: FolderOrFile[];
  className: string;
};

type FolderState = {
  contents: FolderOrFile[];
  collapsed: boolean;
};

class FolderComponent extends React.Component<FolderProps, FolderState> {
  constructor(props: FolderProps) {
    super(props);
    this.state = { contents: props.contents, collapsed: true };
  }

  render() {
    const { name, path, icon, className } = this.props;
    const { contents, collapsed } = this.state;

    //@ts-ignore
    const toggleFolder = (e, p) => {
      this.setState((prevState) => {
        return { ...prevState, collapsed: !this.state.collapsed };
      });

      if (contents.length === 0) {
        get_content_from_folder(path)
          .then((res: Folder) => {
            //@ts-ignore
            this.setState((prevState) => {
              return { ...prevState, contents: res.contents };
            });
          })
          .catch((err) => {
            console.error(err);
          });
      }
    };

    return (
      <div
        key={path}
        className={`folder ${name} ${className} ${
          collapsed ? "collapsed" : ""
        }`}
      >
        <span onClick={(e) => toggleFolder(e, path)} className="folder-name">
          {`${icon}${name}`}
        </span>
        {collapsed
          ? ""
          : contents.map((c) => {
              if ("contents" in c) {
                return (
                  <FolderComponent
                    name={c.name}
                    path={c.path}
                    icon={"📂"}
                    contents={c.contents}
                    className=""
                  />
                );
              } else {
                return (
                  <FileComponent name={c.name} path={c.path} icon={"📄"} />
                );
              }
            })}
      </div>
    );
  }
}

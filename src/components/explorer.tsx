import React, { useState } from "react";
import { File, Folder, FolderOrFile } from "../types/types";
import "../styles/explorer.scss";
import { get_content_from_folder, open_file } from "./backend_api";

export class Explorer extends React.Component {
  //@ts-ignore
  constructor(props) {
    super(props);
    this.state = { files: null, isLoading: true, open_files: [] };
  }

  componentDidMount(): void {
    let root = "C:\\Users\\nilsi\\Documents\\nmide";

    get_content_from_folder(root)
      .then((res) => {
        //@ts-ignore
        this.setState({ files: res, isLoading: false, open_file: [] });
      })
      .catch((err) => {
        this.setState({ files: null, isLoading: true, open_file: [] });
        console.error(err);
      });
  }

  render() {
    //@ts-ignore
    const { isLoading, files } = this.state;

    const handleOpenFile = (fn: string): string => {
      return "";
    };

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
          onFileOpenEvent={handleOpenFile}
        />
      </section>
    );
  }
}

type FileProps = {
  name: string;
  path: string;
  icon: "📄";
  onOpenFileEvent: (fn: string) => string;
};

class FileComponent extends React.Component<FileProps> {
  render() {
    const { name, path, icon } = this.props;

    return (
      <div key={path} className={`file ${name}`}>
        <span
          className="file-name"
          onClick={() => this.props.onOpenFileEvent(path)}
        >{`${icon}${name}`}</span>
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
  onFileOpenEvent: (fn: string) => string;
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
                    onFileOpenEvent={this.props.onFileOpenEvent}
                  />
                );
              } else {
                return (
                  <FileComponent
                    name={c.name}
                    path={c.path}
                    icon={"📄"}
                    onOpenFileEvent={this.props.onFileOpenEvent}
                  />
                );
              }
            })}
      </div>
    );
  }
}

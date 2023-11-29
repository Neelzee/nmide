import React from "react";
import { Folder, FolderOrFile } from "../../types/types";
import { get_content_from_folder } from "../backend_api";
import { FileComponent, FileState } from "./File";

type FolderProps = {
  name: string;
  path: string;
  icon: "📂";
  contents: FolderOrFile[];
  onFileChange: (state: FileState) => void;
};

type FolderState = {
  contents: FolderOrFile[];
  collapsed: boolean;
};

export class FolderComponent extends React.Component<FolderProps, FolderState> {
  constructor(props: FolderProps) {
    super(props);
    this.state = { contents: props.contents, collapsed: true };
  }

  render() {
    const { name, path, icon } = this.props;
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
        className={`folder ${name} ${collapsed ? "collapsed" : ""}`}
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
                    onFileChange={this.props.onFileChange}
                  />
                );
              } else {
                return (
                  <FileComponent
                    name={c.name}
                    path={c.path}
                    icon={"📄"}
                    onFileChange={this.props.onFileChange}
                  />
                );
              }
            })}
      </div>
    );
  }
}

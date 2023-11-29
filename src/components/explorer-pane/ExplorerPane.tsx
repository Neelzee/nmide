import React from "react";
import { get_content_from_folder } from "../backend_api";
import { Folder, File } from "../../types/types";
import { FileComponent, FileState } from "./File";
import { FolderComponent } from "./Folder";
import { WorkspaceContext } from "../../App";

type Props = {
  // Path
  root: string;
  // Array of paths
  open_files: File[];
};

type ExplorerState = {
  root: string;
  open_files: File[];
  root_folder: Folder;
};

export class ExplorerPane extends React.Component<Props, ExplorerState> {
  constructor(props: Props) {
    super(props);
    this.state = {
      ...props,
      root_folder: { name: "", path: props.root, contents: [] },
    };
  }

  componentDidMount() {
    const { root } = this.state;
    get_content_from_folder(root).then((res) => {
      this.setState((prevState) => {
        return { ...prevState, root_folder: res };
      });
    });
  }

  render() {
    const { root_folder } = this.state;

    const fileChange = (state: FileState) => {
      if (state.is_open) {
        this.setState((prevState) => {
          return {
            ...prevState,
            open_files: [
              ...this.state.open_files,
              { name: state.name, path: state.path },
            ],
          };
        });
      } else if (
        this.state.open_files.findIndex((s) => {
          return s.name === state.name && s.path === state.path;
        }) !== -1
      ) {
        this.setState((prevState) => {
          let f = this.state.open_files.filter((s) => {
            return s.name !== state.name && s.path !== state.path;
          });
          return {
            ...prevState,
            open_files: f,
          };
        });
      }
    };

    return (
      <WorkspaceContext.Consumer>
        {(workspace) => (
          <section className="explorer">
            {root_folder === undefined ||
            root_folder.name === undefined ||
            root_folder.name === "." ? (
              <></>
            ) : (
              <span>{root_folder.name}</span>
            )}
            {root_folder.contents.map((fof) => {
              if ("contents" in fof) {
                return (
                  <FolderComponent
                    name={fof.name}
                    path={fof.path}
                    icon={"📂"}
                    contents={fof.contents}
                    onFileChange={(fs) => {
                      fileChange(fs);

                      let of = workspace.open_files;

                      // Add it to the list if it isnt in it.
                      if (
                        fs.is_open &&
                        of.find((p) => {
                          return p.name === fs.name && p.path === fs.path;
                        }) === undefined
                      ) {
                        of.push({ name: fs.name, path: fs.path });
                      }

                      // else, if its not open, and in the list, remove it

                      if (
                        !fs.is_open &&
                        of.find((p) => {
                          return p.name === fs.name && p.path === fs.path;
                        }) !== undefined
                      ) {
                        let i = of.findIndex((p) => {
                          return p.name === fs.name && p.path === fs.path;
                        });

                        of.splice(i, 1);
                      }

                      workspace.setWorkspace({ ...workspace, open_files: of });
                    }}
                  />
                );
              }
              return (
                <FileComponent
                  name={fof.name}
                  path={fof.path}
                  icon={"📄"}
                  onFileChange={(fs) => {
                    fileChange(fs);

                    let of = workspace.open_files;

                    // Add it to the list if it isnt in it.
                    if (
                      fs.is_open &&
                      of.find((p) => {
                        return p.name === fs.name && p.path === fs.path;
                      }) === undefined
                    ) {
                      of.push({ name: fs.name, path: fs.path });
                    }

                    // else, if its not open, and in the list, remove it

                    if (
                      !fs.is_open &&
                      of.find((p) => {
                        return p.name === fs.name && p.path === fs.path;
                      }) !== undefined
                    ) {
                      let i = of.findIndex((p) => {
                        return p.name === fs.name && p.path === fs.path;
                      });

                      of.splice(i, 1);
                    }

                    workspace.setWorkspace({ ...workspace, open_files: of });
                  }}
                />
              );
            })}
          </section>
        )}
      </WorkspaceContext.Consumer>
    );
  }
}

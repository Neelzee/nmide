import React from "react";
import "../styles/menubar.scss";
import { open } from "@tauri-apps/api/dialog";
import { WorkspaceContext } from "../App";

type Props = {
  root: string;
};

type MenuBarState = {
  root: string;
  showFileMenu: boolean;
};

export class MenuBar extends React.Component<Props, MenuBarState> {
  constructor(props: Props) {
    super(props);
    this.state = {
      showFileMenu: false,
      root: props.root,
    };
  }

  toggleFileMenu = () => {
    this.setState((prevState) => ({
      //@ts-ignore
      showFileMenu: !prevState.showFileMenu,
    }));
  };

  render() {
    //@ts-ignore
    const { showFileMenu } = this.state;

    return (
      <WorkspaceContext.Consumer>
        {(workspace) => (
          <nav>
            <div>
              <span className="logo">NM</span>
            </div>
            <div>
              <span className="btn" onClick={this.toggleFileMenu}>
                File
              </span>
              {showFileMenu && (
                //@ts-ignore
                <FileMenu setWorkspace={workspace.setWorkspace} />
              )}
            </div>
            <div>
              <span className="btn">Edit</span>
            </div>
            <div>
              <span className="btn">View</span>
            </div>
            <div>
              <span className="btn">Settings</span>
            </div>
            <div>
              <span className="btn">Help</span>
            </div>
          </nav>
        )}
      </WorkspaceContext.Consumer>
    );
  }
}

function FileMenu(props: { setWorkspace: () => {} }, workspace: any) {
  const openFolder = () => {
    open({
      directory: true,
      multiple: false,
    })
      .then((res) => {
        //@ts-ignore
        setWorkspace({ ...workspace, root: res });
      })
      .catch((err) => console.error(err));
  };

  return (
    <article className="filemenu">
      <p>New File</p>
      <p>Open File</p>
      <p onClick={openFolder}>Open Folder</p>
      <p>Save</p>
      <p>Save As</p>
      <p>Exit</p>
    </article>
  );
}

import React, { useState } from "react";
import "../styles/menubar.scss";
import { open } from "@tauri-apps/api/dialog";

export class MenuBar extends React.Component {
  //@ts-ignore
  constructor(props) {
    super(props);
    this.state = {
      showFileMenu: false,
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
      <>
        <nav>
          <div>
            <span className="logo">NM</span>
          </div>
          <div>
            <span className="btn" onClick={this.toggleFileMenu}>
              File
            </span>
            {showFileMenu && <FileMenu />}
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
      </>
    );
  }
}

function FileMenu() {
  const openFolder = () => {
    open({
      directory: true,
      multiple: true,
    })
      .then((res) => console.log(res))
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

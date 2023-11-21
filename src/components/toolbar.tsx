import React from "react";
import "../styles/toolbar.scss";

export class ToolBar extends React.Component {
  render() {
    return (
      <>
        <nav>
          <div>File</div>
          <div>Edit</div>
          <div>View</div>
          <div>Settings</div>
          <div>Help</div>
        </nav>
      </>
    );
  }
}

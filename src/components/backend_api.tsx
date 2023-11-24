import { File, Folder, FolderOrFile } from "../types/types";
import { invoke } from "@tauri-apps/api";

export function open_file(path: string): Promise<string> {
  return invoke("open_file", { path: path });
}

export function get_content_from_folder(path: string): Promise<Folder> {
  return invoke("get_content_in_folder", { root: path }).then((res) => {
    let r = parse_backend_fof(res);
    if (r === undefined) {
      return {
        name: path,
        path: path,
        contents: [],
      };
    } else {
      return "contents" in r
        ? r
        : { name: "Root", path: `../${path}`, contents: [r] };
    }
  });
}

function parse_backend_fof(object: any): FolderOrFile | undefined {
  if ("Folder" in object) {
    let folder: Folder = object.Folder;
    //@ts-ignore
    folder.contents = folder.contents
      .map((fof) => parse_backend_fof(fof))
      .filter((fof) => fof !== undefined);
    return folder;
  }

  if ("File" in object) {
    let file: File = object.File;
    return file;
  }
}

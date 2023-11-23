import { File, Folder, FolderOrFile } from "../types/types";
import { invoke } from "@tauri-apps/api";

export function foo() {}

export function get_content_from_folder(path: string): Promise<Folder> {
  //@ts-ignore
  return invoke("get_content_in_folder", { root: path })
    .then((res) => {
      //@ts-ignore
      return parse_backend_fof(res);
    })
    .catch((err) => {
      throw err;
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

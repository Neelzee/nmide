import "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import * as A from "fp-ts/Array";
import * as S from "fp-ts/string";
import { DirEntry, readDir } from "@tauri-apps/plugin-fs";
import { appDataDir, join } from "@tauri-apps/api/path";
import { convertFileSrc } from "@tauri-apps/api/core";

export const getPaths = async (): Promise<string[]> => {
  const pluginDir = await appDataDir()
    .then(p => join(p, "plugins"));
  return readDir(pluginDir)
    .then(dirs => pipe(
      dirs,
      A.filter(d => d.isFile),
    ))
    .then(A.map<DirEntry, Promise<string>>(p => join(pluginDir, p.name)))
    .then(A.map<Promise<string>, Promise<string>>(p => p.then(convertFileSrc)))
    .then(paths => Promise.all(paths))
    .then(A.sort(S.Ord));
};

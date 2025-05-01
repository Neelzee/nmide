import { exec } from "child_process";
import * as fs from "fs";
import toml from "toml";
import util from "util";
import path from "node:path";

const data = fs.readFileSync("./Modules.toml", "utf-8");

type Module = {
  name: string
  path: string,
  kind: string,
  enabled: boolean,
  "package-manager"?: string,
};

type TomlData = {
  modules: { [key in string]?: Omit<Partial<Module>, "name"> }
};

let tomlData: TomlData | undefined = undefined;
try {
  tomlData = toml.parse(data);
} catch (e) {
  console.error(e);
}

if (tomlData === undefined) {
  throw new Error("tomlData is undefined");
};

const parseData = ({ modules: data }: TomlData): Module[] =>
  Object.keys(data)
    .map((k): Partial<Omit<Module, "name">> & { name: string } => {
      return { ...data[k], name: k };
    })
    .filter(m => m?.enabled === undefined ? true : m.enabled)
    .filter(m => m?.kind !== undefined)
    .filter(m => m!.kind !== "rs")
    .filter(m => m!.kind !== "css")
    .map((m): Module => {
      const { name, path, kind } = m;
      return {
        name,
        path: path === undefined
          ? `../modules/${name}`
          : path,
        kind: kind!,
        enabled: true,
        "package-manager": m["package-manager"]
      };
    });


export const install = () => {
  parseData(tomlData).forEach(m => {
    require(m.path);
  });
};
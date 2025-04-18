import * as fs from "fs";

const modules_regx = /.*\s=\s{.*}/gm;
const table_regx = /{.*}/gm
const path_regx = /path\s=\s".*"/gm
const kind_regx = /kind\s=\s".*"/gm

const data = fs.readFileSync("./Modules.toml", "utf-8");

type Module = {
  name: string
  path?: string,
  kind?: string,
}

const modules: Module[] = modules_regx[Symbol.match](data)?.map(s => {
  const name = s.split("=")[0]?.trim();
  const table = table_regx[Symbol.match](s)?.at(0)!;
  const path = path_regx[Symbol.match](table)?.at(0);
  const kind = kind_regx[Symbol.match](table)?.at(0);
  return { name: name === undefined ? "" : name,
           path: path === undefined
             ? ""
             : path.split("=")[1]?.trim()
               .replace("\"", "")
               .replace("\"", ""),
           kind: kind === undefined
             ? "rs"
             : kind.split("=")[1]?.trim()
                .replace("\"", "")
                .replace("\"", ""),
         };
})!;

console.log(modules);
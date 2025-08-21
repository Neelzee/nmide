import { isTObj, type State, type Value } from "@nmide/js-utils";

type TreeData = {
  name: string,
  children: TreeData[],
};

export const parseData = (data: State): TreeData => {
  return {
    name: "root",
    children: Object.keys(data).map(name => {
      const v = data[name];
      return {
        name,
        children: isTObj(v)
          ? parseData(v.obj).children
          : [{ name: JSON.stringify(v), children: [] }]

      };
    }),
  };
}
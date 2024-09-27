import { TMap } from "./bindings/TMap";

const ModelFold = ({ map: a }: TMap, { map: b }: TMap): TMap => {
  return {
    map: a.concat(
      b.filter(
        ([bk, _]) =>
          a.find(([ak, _]) => ak === bk)
          !== undefined
      )
    )
  };
};

export default ModelFold;

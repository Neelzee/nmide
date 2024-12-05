import { Json } from "fp-ts/lib/Json";
import { TMap, TValue } from "./bindings/TMap";
import { tBool, tFloat, tInt, tList, tStr } from "./Utils";

export default class HtmlBuilder {
  map: [string, TValue][]

  addField(key: string, val: TValue): HtmlBuilder {
    this.map.push([key, val]);
    return this;
  }

  addInt(key: string, val: number): HtmlBuilder {
    this.map.push([key, tInt(val)]);
    return this;
  }

  addFloat(key: string, val: number): HtmlBuilder {
    this.map.push([key, tFloat(val)]);
    return this;
  }

  addBool(key: string, val: boolean): HtmlBuilder {
    this.map.push([key, tBool(val)]);
    return this;
  }

  addStr(key: string, val: string): HtmlBuilder {
    this.map.push([key, tStr(val)]);
    return this;
  }

  addList(key: string, val: TValue[]): HtmlBuilder {
    this.map.push([key, tList(val)]);
    return this;
  }

  addObj(key: string, val: TValue[]): HtmlBuilder {
    this.map.push([key, tList(val)]);
    return this;
  }

  build(): TMap {
    return [];
  }
}

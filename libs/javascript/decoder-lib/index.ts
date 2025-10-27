/**
  * @package decoder-lib
  *
  * @description Contains the necessary type descriptions for `io-ts` to be able
  * to do runtime type validation. Due to the nested recursive types, there are
  * a lot of duplicated code, making this hard to maintain.
  *
  * @author Nils Michael <nilsien2001@gmail.com>
  *
  * @see [io-ts](https://github.com/gcanti/io-ts/blob/master/index.md)
  */


export * from "./lib/event_decoder";
export * from "./lib/html_decoder";
export * from "./lib/value_decoder";
export * from "./lib/core_modification_decoder";
export * from "./lib/module_decoder";
export * from "./lib/instr_decoder";
import pr from "io-ts-reporters";
export const prettyReport = pr;

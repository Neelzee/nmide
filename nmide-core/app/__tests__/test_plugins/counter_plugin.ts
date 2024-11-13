import {
  TMap,
  TValue,
  TMsg,
  HtmlBuilder,
  NmluginVerified as Nmlugin,
  lookup,
  TValueInt,
} from "@nmide/js-utils";
import * as O from "fp-ts/Option";
import { pipe } from "fp-ts/lib/function";

const CounterPlugin: Nmlugin = {
  init: () => [["counter", { "Int": 0 }]],
  view: (model: TMap) => {
    const counter = pipe(
      lookup<string, TValue>("counter")(model),
      O.getOrElse<TValue>(() => { return { Int: -1 } }),
    ) as TValueInt;
    return new HtmlBuilder()
      .kind("Div")
      .kids([
        new HtmlBuilder()
          .kind("Text")
          .text(`Count: ${counter.Int}`),
        new HtmlBuilder()
          .kind("Button")
          .attrs([{ OnClick: { Msg: ["increment", { Int: 1 }] } }]),
      ]).build();
  },
  update: (msg: TMsg, model: TMap) => {
    const [tmsg, tvalue] = msg.Msg;
    if (tmsg !== "increment") return [];
    const value = (pipe(
      lookup<string, TValue>("counter")(model),
      O.getOrElse<TValue>(() => { return { Int: 0 } }),
    ) as TValueInt).Int;
    const increment = (tvalue as TValueInt).Int;
    return [["counter", { Int: value + increment }]];
  },
};

export default CounterPlugin;

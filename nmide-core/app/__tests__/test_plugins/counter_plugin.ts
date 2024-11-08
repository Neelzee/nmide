import { TMap, TValue } from "../../lib/bindings/TMap";
import { TMsg } from "../../lib/bindings/TMsg";
import HtmlBuilder from "../../lib/HtmlBuilder";
import { NmluginVerified as Nmlugin } from "../../lib/Nmlugin";
import * as O from "fp-ts/Option";
import * as U from "../../lib/Utils";
import { pipe } from "fp-ts/lib/function";

const CounterPlugin: Nmlugin = {
  init: () => [["counter", { "Int": 0 }]],
  view: (model: TMap) => {
    const counter = pipe(
      U.lookup<string, TValue>("counter")(model),
      O.getOrElse<TValue>(() => { return { Int: -1 } }),
    ) as U.TValueInt;
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
      U.lookup<string, TValue>("counter")(model),
      O.getOrElse<TValue>(() => { return { Int: 0 } }),
    ) as U.TValueInt).Int;
    const increment = (tvalue as U.TValueInt).Int;
    return [["counter", { Int: value + increment }]];
  },
};

export default CounterPlugin;

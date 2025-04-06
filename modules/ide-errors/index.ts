import "@nmide/js-utils";
import { isTObj, THtml, tList, tLookupOr, TMap, TMsg, tObj, tObjLookup, TValueList, TValueStr } from "@nmide/js-utils";
import * as A from "fp-ts/Array";
import { pipe } from "fp-ts/lib/function";

let init = false;

window.plugins.set(
  "ide-errors",
  {
    init: (): TMap => {
      if (init) return [["ide-errors", { list: [] }]];
      init = true;
      window.log.error = (msg, ...xs) => {
        console.log("unhandled error inputs: ", xs);
        window.emit("msg", { msg: ["general-error", tObj(["info", msg])] });
      };

      return [["ide-errors", { list: [] }]];
    },
    update: (msg: TMsg, model: TMap): TMap => {
      if (msg.msg[0].includes("error")) {
        const errors = tLookupOr<TValueList>("ide-errors")(tList([]))(model);
        errors.list.push(msg.msg[1]);
        return [["ide-errors", errors]];
      }
      return [];
    },
    view: (model: TMap): THtml => {
      const errors = tLookupOr<TValueList>("ide-errors")(tList([]))(model)
        .list
        .filter(isTObj);
      return {
        kind: "div",
        attrs: [{ id: "location-status-bar" }],
        kids: pipe(
          errors,
          A.filterMap(tObjLookup<TValueStr>("info")),
          A.map(s => {
            return {
              kind: "p",
              attrs: [],
              kids: [],
              text: s.str
            };
          })
        ),
        text: null,
      }
    },
  }
)

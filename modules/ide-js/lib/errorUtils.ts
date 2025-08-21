import {
  getValue,
  isTObj,
  tObj,
  tObjLookupOrType,
  tObjLookupUnd,
  tStr,
  type Value,
  type ValueObj,
  type ValueStr
} from "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";

export type IDEError = {
  module: string,
  msg?: string,
  error?: string,
  triggeringEvent?: string
};

export const mkError = (
  module: string,
  msg?: string,
  error?: string,
  triggeringEvent?: string
): ValueObj => tObj({
  module,
  msg,
  error,
  triggeringEvent
});


const toError = (obj: ValueObj): IDEError | undefined => pipe(
  obj,
  tObjLookupOrType<ValueStr>("module")(tStr("")),
  getValue,
  module => {
    return {
      module,
      msg: pipe(
        obj,
        tObjLookupUnd<ValueStr>("module"),
        msg => msg && getValue(msg),
      ),
      error: pipe(
        obj,
        tObjLookupUnd<ValueStr>("error"),
        error => error && getValue(error),
      ),
      triggeringEvent: pipe(
        obj,
        tObjLookupUnd<ValueStr>("triggeringEvent"),
        triggeringEvent => triggeringEvent && getValue(triggeringEvent),
      ),
    };
  },
);


export const dcError = (value: Value | null): IDEError | undefined =>
  value === null
    ? undefined
    : isTObj(value)
      ? toError(value)
      : undefined

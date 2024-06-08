import { NmideError } from "./models/NmideError";
import { NmideReport } from "./models/NmideReport";

export function split_with_err<T>(err: NmideError): [T, NmideReport?] {
  return [err.val, err.rep];
}


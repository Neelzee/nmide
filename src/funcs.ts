import { NmideReport, NmideError } from "./types.ts";

export function split_with_err<T>(err: NmideError<T>): [T, NmideReport | null] {
  return [err.val, err.rep];
}


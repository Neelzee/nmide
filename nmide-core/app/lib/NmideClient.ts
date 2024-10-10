import { invoke, InvokeOptions } from "@tauri-apps/api/core"
import { TMap } from "./bindings/TMap";
import * as t from "io-ts";
import * as E from "fp-ts/Either";
import { pipe } from "fp-ts/lib/function";
import { DHtml } from "./Decoder";

type Cmd = {
  "install_plugins": {
    arg: undefined,
  },
  "uninstall_plugins": {
    arg: undefined,
  }
  "init": {
    arg: { tmodel: TMap },
  },
  "view": {
    arg: { model: TMap },
  },
}

const DHtmlArr = t.array(DHtml);

const NmideDecoder = {
  "install_plugins": t.null,
  "uninstall_plugins": t.null,
  "init": t.null,
  "view": DHtmlArr,
};

const NmideClient = async <K extends keyof Cmd>(
  cmd: K,
  args: Cmd[K]["arg"],
  options?: InvokeOptions,
): Promise<E.Either<Error, t.TypeOf<typeof NmideDecoder[K]>>> => {
  try {
    const u = await invoke(cmd, args, options);
    return pipe(
      NmideDecoder[cmd].decode(u),
      E.mapLeft(errs => new Error(
        `Failed parsing response from backend: ${errs.join("\n")}`
      )));
  } catch (err) {
    return E.left<Error, t.TypeOf<(typeof NmideDecoder)[K]>>(new Error(`${err}`));
  }
}

export default NmideClient;

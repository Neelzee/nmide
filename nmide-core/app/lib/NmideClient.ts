import { invoke, InvokeArgs, InvokeOptions } from "@tauri-apps/api/core"
import * as t from "io-ts";
import * as E from "fp-ts/Either";
import { pipe } from "fp-ts/lib/function";
import { DHtmlArr, DMapArr } from "./Decoder";
import { PathReporter } from "io-ts/PathReporter";
import { TMap } from "./bindings/TMap";
import { TMsg } from "./bindings/TMsg";

type NmideArgs = {
  "install": {
    args: undefined,
  }
  "init": {
    args: undefined,
  },
  "view": {
    args: { tmodel: TMap },
  },
  "update": {
    args: { tmsg: TMsg, tmodel: TMap, },
  },
}

const NmideDecoderTest = {
  "install": t.null,
  "init": DMapArr,
  "view": DHtmlArr,
  "update": DMapArr,
}

type NmideDecodedType<
  K extends keyof NmideArgs
  & keyof typeof NmideDecoderTest
> = t.TypeOf<typeof NmideDecoderTest[K]>

const NmideClient = async <
  K extends keyof NmideArgs & keyof typeof NmideDecoderTest,
  A extends InvokeArgs & NmideArgs[K]["args"]
>(
  cmd: K,
  args?: A,
  options?: InvokeOptions,
): Promise<E.Either<Error, NmideDecodedType<K>>> => {
  try {
    return pipe(
      await invoke(cmd, args, options),
      u => NmideDecoderTest[cmd].decode(u),
      decoded =>
        E.isLeft(decoded)
          ? E.left(
            new Error(
              "Failed validating data from backend: " +
              `${PathReporter.report(decoded).join("\n")}`
            )
          )
          : E.right(decoded.right),
    )
  } catch (error) {
    return E.left(new Error(`Got error from backend: ${error}`))
  }
}


export default NmideClient;

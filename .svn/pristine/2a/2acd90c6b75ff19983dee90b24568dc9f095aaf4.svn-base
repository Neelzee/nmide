import { App } from "../core/app/App";
import { renderHtml, parseHtml } from "../core/app/lib/renderHtml";
import { jspInstaller } from "../core/app/lib/jspInstaller";
import { cssInstaller } from "../core/app/lib/cssInstaller";
import * as E from "fp-ts/Either";
import { NmideArgs, NmideClient, NmideDecodedType, NmideDecoder } from "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import * as t from "io-ts";

const listenHandler: Map<string, any> = new Map();

const url = "http://localhost:8080/";

const client: NmideClient =
  <
    K extends keyof NmideArgs & keyof typeof NmideDecoder,
    A extends NmideArgs[K]["args"]
  >(
    cmd: K,
    args: A
  ) => {
    return fetch(`${url}${cmd}`, {
      method: "post",
      headers: {
        'Accept': 'application/json',
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(args)
    })
      .then(r => r.json())
      .then(r => { console.log("Res: ", r); return r; })
      .then(E.right)
      .catch(err => E.left(new Error(err)))
      .then(
        E.match<Error, unknown, E.Either<Error, NmideDecodedType<K>>>(
          E.left,
          unknown_data => pipe(
            unknown_data,
            NmideDecoder[cmd].decode,
            E.match<t.Errors, NmideDecodedType<K>, E.Either<Error, NmideDecodedType<K>>>(
              errs => E.left(
                new Error(
                  `Error from validating backend: ${JSON.stringify(errs)}`
                  + `, supplied data: ${JSON.stringify(unknown_data)}`
                )
              ),
              data => E.right(data),
            ),
          )
        )
      )
  };

document.addEventListener("DOMContentLoaded", () => {
  App({
    renderHtml,
    parseHtml,
    client,
    listen: (x, handler) => {
      listenHandler.set(x, handler);
      return new Promise(r => r(null));
    },
    emit: (evt, payload) => {
      const handler = listenHandler.get(evt);
      if (handler !== undefined) {
        handler({ payload });
      }
      return new Promise(r => r());
    },
    getPluginPaths: new Promise(r => r([
      "./plugins/counter.js",
    ])),
    pluginInstallers: [jspInstaller, cssInstaller]
  });
})


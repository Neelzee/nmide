import "@nmide/js-utils";
import {
  Decoder,
  emptyHtml,
  GroupBy,
  NmluginVerified as Nmlugin,
  NmluginUnknown,
  THtml,
  TMap,
  TMsg
} from "@nmide/js-utils";
import { pipe } from "fp-ts/lib/function";
import * as A from "fp-ts/Array";
import { toArray, fromFoldable } from "fp-ts/lib/Map";
import { Ord, Eq } from "fp-ts/string";
import { snd } from "fp-ts/lib/Tuple";
import * as O from "fp-ts/Option";
import { fromEquals } from "fp-ts/Eq";

let pluginViews: [string, ((model: TMap) => unknown)][] = [];

const pluginName = "ide-view";

const init = (): TMap => {
  pluginViews = pipe(
    window.plugins,
    toArray(Ord),
    A.filter(([s, _]) => s !== pluginName),
    A.map<[string, NmluginUnknown], [string, ((model: TMap) => unknown)]>(
      ([s, p]) => ([s, p.view])
    ),
  );
  window.plugins = pipe(
    window.plugins,
    toArray(Ord),
    A.map<[string, NmluginUnknown], [string, NmluginUnknown]>(
      ([s, p]) => {
        if (s === pluginName) return [s, p];
        return ([s, { ...p, view: (_: TMap) => emptyHtml() }]);
      }
    ),
    fromFoldable(Eq, { concat: (x, _) => x }, A.Foldable)
  );
  return [];
};

const update = (_: TMsg, __: TMap): TMap => {
  return [];
};

const getLocation = (html: THtml): string => {
  const locAttr = html.attrs.find(
    attr => "class" in attr && attr.class.includes("location-")
  );
  if (locAttr !== undefined && "class" in locAttr) {     // locAttr.class === "foo bar location-locationName foobar"
    const index = locAttr.class.indexOf("location-");    // index = 7
    if (index === -1) return "";                         // locAttr.substring(7) === "location-locationName foobar"
    return locAttr.class.substring(index).split(" ")[0]; // locAttr.substring(7).split(" ")[0] === "location-locationName"
  }
  return "";
};

const view = (model: TMap) => {
  pipe(
    pluginViews,
    A.map<[string, ((model: TMap) => unknown)], [string, unknown]>(([s, v]) => [s, v(model)]),
    A.filterMap(([s, u]) => pipe(
      u,
      Decoder.DHtml.decode,
      O.fromEither,
      O.map<THtml, [string, THtml]>(h => [s, h]),
    )),
    GroupBy(fromEquals((x, y) => getLocation(snd(x)) === getLocation(snd(y)))),
    A.map<[string, THtml][], [string, [string, THtml][]]>(xs =>
      [
        pipe(
          A.head(xs),
          O.map(([_, x]) => getLocation(x)),
          O.getOrElse(() => ""),
        ),
        xs
      ]
    )
  ).forEach(([location, htmls]) => {
    let loc = location.split("location-")[1];
    loc = loc === undefined ? "" : loc;
    let rootElement = document.getElementById(loc);
    if (rootElement === null) {
      rootElement = window.root;
    }
    htmls.forEach(([pln, h]) => {
      const elem = window.parseHtml(h);
      if (elem === undefined) return;
      rootElement.appendChild(elem);
      window.cleanup.push([pln, (() => { rootElement.removeChild(elem); })]);
    })
  });
  return emptyHtml();
};

const plugin: Nmlugin = {
  init,
  update,
  view
};

window.plugins.set(
  pluginName,
  //@ts-ignore
  { ...plugin, parseHtml },
);

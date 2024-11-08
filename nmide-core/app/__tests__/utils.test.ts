import { expect, test } from 'vitest';
import * as U from "../lib/Utils";
import * as O from "fp-ts/Option";
import * as A from "fp-ts/Array";
import { TMap } from '../lib/bindings/TMap';

[
  {
    list: [],
    key: "foo",
    found: false,
  },
  {
    list: [["bar", "foo"]] as [string, any][],
    key: "foo",
    found: false,
  },
].forEach(({ list, key, found }) => {
  test(`Lookup: ${list}, ${key}, ${found ? "exists" : "not exists"}`, () => {
    if (found) {
      expect(O.isSome(U.lookup(key)(list))).toBeTruthy();
    } else {
      expect(O.isNone(U.lookup(key)(list))).toBeTruthy();
    }
  });
});

[
  {
    a: ["foo", ""] as [string, any],
    b: ["bar", ""] as [string, any],
    equal: false,
  },
].forEach(
  ({ a, b, equal }) => test(
    `NmluginEq: ${a} eq ${b} => ${equal}`,
    () => expect(U.NmluginEq.equals(a, b)).toBe(equal)
  )
);

const testDataModelFold: readonly { xs: TMap, ys: TMap, zs: TMap }[] = [
  {
    xs: [["foo", { Int: 1, }]],
    ys: [["bar", { Int: 0 }]],
    zs: [["foo", { Int: 1 }], ["bar", { Int: 0 }]],
  },
  {
    xs: [["foo", { Int: 1 }]],
    ys: [["foo", { Int: 0 }]],
    zs: [["foo", { Int: 1 }], ["foo", { Int: 1 }]],
  },
];

testDataModelFold.forEach(({ xs, ys, zs }) => {
  test(`ModelFold is associative: xs: [${xs.join(", ")}], ys: [${ys.join(", ")}], zs: [${zs.join(", ")}]`, () => {
    const xs_fold_ys = U.PluginMonoid.concat(xs, ys);
    const ys_fold_xs = U.PluginMonoid.concat(ys, xs);
    zs.forEach(el => {
      expect(A.elem(U.TMapFieldEq)(el)(xs_fold_ys)).toBeTruthy();
      expect(A.elem(U.TMapFieldEq)(el)(ys_fold_xs)).toBeTruthy();
    });
  });
  test(`Identity should hold in ModelFold: xs: [${xs.join(", ")}], ys: [${ys.join(", ")}], zs: [${zs.join(", ")}]`, () => {
    expect(U.PluginMonoid.concat(xs, U.PluginMonoid.empty), "xs fold []")
      .toStrictEqual(xs);
    expect(U.PluginMonoid.concat(ys, U.PluginMonoid.empty), "ys fold []")
      .toStrictEqual(ys);
    expect(U.PluginMonoid.concat(zs, U.PluginMonoid.empty), "zs fold []")
      .toStrictEqual(U.TMapSort(zs));
    expect(U.PluginMonoid.concat(U.PluginMonoid.empty, xs), "[] fold xs")
      .toStrictEqual(xs);
    expect(U.PluginMonoid.concat(U.PluginMonoid.empty, ys), "[] fold ys")
      .toStrictEqual(ys);
    expect(U.PluginMonoid.concat(U.PluginMonoid.empty, zs), "[] fold zs")
      .toStrictEqual(U.TMapSort(zs));
  });
});

const testDataModelOverwrite: readonly { prevModel: TMap, newModel: TMap, finalModel: TMap }[] = [
  {
    prevModel: [["foo", { Int: 1, }]],
    newModel: [["bar", { Int: 0 }]],
    finalModel: [["foo", { Int: 1 }], ["bar", { Int: 0 }]],
  },
  {
    prevModel: [["foo", { Int: 1 }]],
    newModel: [["foo", { Int: 0 }]],
    finalModel: [["foo", { Int: 0 }]],
  },
  {
    prevModel: [["foo", { Int: 1 }]],
    newModel: [],
    finalModel: [["foo", { Int: 1 }]],
  },
  {
    prevModel: [],
    newModel: [["foo", { Int: 1 }]],
    finalModel: [["foo", { Int: 1 }]],
  },
];


testDataModelOverwrite.forEach(({ prevModel, newModel, finalModel }) => {
  test(`ModelOverwrite:\nprevModel: [${U.TMapToString(prevModel)}]\nnewModel: [${U.TMapToString(newModel)}]\nfinalModel: [${U.TMapToString(finalModel)}]`, () => {
    expect(U.ModelOverwrite(prevModel, newModel).sort()).toStrictEqual(finalModel.sort());
  });
});

import { expect, suite, test } from 'vitest';
import * as U from "../lib/Utils";
import * as O from "fp-ts/Option";
import * as A from "fp-ts/Array";
import { NmluginEq, TotalTMapFieldEq } from '../lib/Eq';
import { TMap, TValue } from '../lib/TMap';
import { TMapToString } from '../lib/Debug';
import { isBool, isFloat, isInt, isList, isObj, isStr, tObj, tValueMaybe, TValuePrimities } from '../lib/Types';

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
    () => expect(NmluginEq.equals(a, b)).toBe(equal)
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
      expect(A.elem(TotalTMapFieldEq)(el)(xs_fold_ys)).toBeTruthy();
      expect(A.elem(TotalTMapFieldEq)(el)(ys_fold_xs)).toBeTruthy();
    });
  });
  test(`Identity should hold in ModelFold: xs: [${xs.join(", ")}], ys: [${ys.join(", ")}], zs: [${zs.join(", ")}]`, () => {
    expect(U.PluginMonoid.concat(xs, U.PluginMonoid.empty), "xs fold []")
      .toStrictEqual(xs);
    expect(U.PluginMonoid.concat(ys, U.PluginMonoid.empty), "ys fold []")
      .toStrictEqual(ys);
    expect(U.PluginMonoid.concat(zs, U.PluginMonoid.empty), "zs fold []")
      .toStrictEqual(A.sort(U.PartialTMapFieldOrd)(zs));
    expect(U.PluginMonoid.concat(U.PluginMonoid.empty, xs), "[] fold xs")
      .toStrictEqual(xs);
    expect(U.PluginMonoid.concat(U.PluginMonoid.empty, ys), "[] fold ys")
      .toStrictEqual(ys);
    expect(U.PluginMonoid.concat(U.PluginMonoid.empty, zs), "[] fold zs")
      .toStrictEqual(A.sort(U.PartialTMapFieldOrd)(zs));
  });
});

const testDataModelOverwrite: readonly {
  prevModel: TMap,
  newModel: TMap,
  finalModel: TMap
}[]
  = [
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
  test(
    `ModelOverwrite:\nprevModel: [${TMapToString(prevModel)}]\nnewModel: [${TMapToString(newModel)}]\nfinalModel: [${TMapToString(finalModel)}]`,
    () => expect(U.ModelOverwrite(prevModel, newModel).sort())
      .toStrictEqual(finalModel.sort())
  );
});

const testTypeInferene: [TValuePrimities, string][] = [
  [1, "Int"],
  ["foobar", "Str"],
  [-1, "Int"],
  [1.2, "Float"],
  [[1, 2, 3, 4, 5], "List"],
  [[["obj_1", 1], ["obj_2", 2]], "Obj"],
];

suite("TypeInference", () => {
  test("Int", () => expect(isInt(1)).toBe(true));
  test("Float", () => {
    expect(isInt(1.2)).toBe(false);
    expect(isFloat(1.2)).toBe(true);
  });
  test("Str", () => expect(isStr("foobar")).toBe(true));
  test("Bool", () => expect(isBool(false)).toBe(true));
  test("List", () => {
    expect(isList([])).toBe(true);
    expect(isList([1, 2, 3, 4, 5])).toBe(true);
    expect(isList([["obj_1", 1], ["obj_2", 2]])).toBe(false);
  });
  test("Obj", () => {
    expect(isObj([])).toBe(false);
    expect(isObj([1, 2, 3, 4, 5])).toBe(false);
    expect(isObj([["obj_1", 1], ["obj_2", 2]])).toBe(true);
  });
});

const tValuePrimitives: [TValuePrimities, string][] = [
  [1, "Int"],
  ["foobar", "Str"],
  [-1, "Int"],
  [1.2, "Float"],
  [[1, 2, 3, 4, 5], "List"],
  [[["obj_1", 1], ["obj_2", 2]], "Obj"],
];

tValuePrimitives.forEach(([x, y]) =>
  test(`tValueMaybe: ${x} to be ${y}`, () => {
    const result = tValueMaybe(x);
    expect(O.isSome(result)).toBe(true);
    expect(y in O.getOrElse<TValue>(() => { throw new Error("Unreachable"); })(result))
      .toBe(true);
  })
);

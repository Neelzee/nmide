import { afterEach, expect, suite, test } from 'vitest';
import { clearMocks } from '@tauri-apps/api/mocks';
import { NmluginVerified as Nmlugin } from "../lib/Nmlugin";
import { TMsg } from "nmide-js-utils/bindings/TMsg";
import { TMap } from "nmide-js-utils/bindings/TMap";
import * as U from "../lib/Utils.ts";
import * as E from "fp-ts/Either";
import { UpdateFunction } from "../lib/Update.tsx";
import CounterPlugin from "./test_plugins/counter_plugin.ts";
import { NmideClientMock } from './init.test.ts';

afterEach(clearMocks);

// == Validates that the UpdateFunction can handle Single-Msg state changing ===

type SingleMsgStateChangeTest = {
  name: string,
  plugins: [string, Nmlugin][],
  tests: {
    desc: string,
    msg: TMsg,
    state: TMap,
    result: TMap,
  }[]
};

const singleMsgStateChangeTestData: SingleMsgStateChangeTest[] = [
  {
    name: "Single Plugin",
    plugins: [["CounterPlugin", CounterPlugin]],
    tests: [
      {
        desc: "Incrementing with 0, should not change the state",
        msg: {
          Msg: ["increment", U.tInt(0)]
        },
        state: [["counter", U.tInt(0)]],
        result: [["counter", U.tInt(0)]],
      },
      {
        desc: "Non-increment messages should be ignore, and have no state change",
        msg: {
          Msg: ["not-increment", U.tInt(1)]
        },
        state: [["counter", U.tInt(0)]],
        result: [],
      },
      {
        desc: "Incrementing with 1, should change the state",
        msg: {
          Msg: ["increment", U.tInt(1)]
        },
        state: [["counter", U.tInt(0)]],
        result: [["counter", U.tInt(1)]],
      },
    ]
  }
];


const mockBackendState = (response: [string, TMap][]) => NmideClientMock("update", response);

singleMsgStateChangeTestData.forEach(({ name, plugins, tests }) => {
  suite(`Plugin: ${name}`, () => tests.forEach(({ desc, msg, state, result }) => {
    test(desc, async () => {
      mockBackendState([]);
      expect(await UpdateFunction(msg, plugins, state)).toStrictEqual(E.right(result));
    });
  }));
});

// ============ Validates that the test Plugins behave as expected =============

type MultiMsgStateChangeTestData = {
  name: string,
  plugins: [string, Nmlugin][],
  tests: {
    desc: string,
    msgs: TMsg[],
    state: TMap,
    result: TMap,
  }[]
}

const multiMsgStateChangeTestData: MultiMsgStateChangeTestData[] = [
  {
    name: "Single Plugin",
    plugins: [["CounterPlugin", CounterPlugin]],
    tests: [
      {
        desc: "Incrementing with 0, should not change the state",
        msgs: [
          { Msg: ["increment", U.tInt(0)] },
          { Msg: ["increment", U.tInt(0)] },
          { Msg: ["increment", U.tInt(0)] },
          { Msg: ["increment", U.tInt(0)] },
          { Msg: ["increment", U.tInt(0)] },
          { Msg: ["increment", U.tInt(0)] },
          { Msg: ["increment", U.tInt(0)] },
          { Msg: ["increment", U.tInt(0)] },
        ],
        state: [["counter", U.tInt(10)]],
        result: [["counter", U.tInt(10)]],
      },
      {
        desc: "Non-increment messages should be ignore, and have no state change",
        msgs: [
          { Msg: ["not-increment", U.tInt(1)] },
          { Msg: ["incrmnt", U.tInt(100)] },
          { Msg: ["foo", U.tInt(3289)] },
          { Msg: ["foobar", U.tInt(332)] },
          { Msg: ["baz", U.tInt(10010)] },
          { Msg: ["!!!jc", U.tInt(31)] },
          { Msg: ["[Object object]", U.tInt(12020202020)] },
        ],
        state: [["counter", U.tInt(0)]],
        result: [["counter", U.tInt(0)]],
      },
      {
        desc: "Incrementing with 1, should change the state",
        msgs: [
          { Msg: ["increment", U.tInt(1)] },
          { Msg: ["increment", U.tInt(2)] },
          { Msg: ["increment", U.tInt(3)] },
          { Msg: ["increment", U.tInt(4)] },
          { Msg: ["increment", U.tInt(5)] },
          { Msg: ["increment", U.tInt(6)] },
          { Msg: ["increment", U.tInt(7)] },
          { Msg: ["increment", U.tInt(8)] },
          { Msg: ["increment", U.tInt(9)] },
        ],
        state: [["counter", U.tInt(0)]],
        result: [["counter", U.tInt(45)]],
      },
    ]
  }
];

multiMsgStateChangeTestData.forEach(({ name, plugins, tests }) => {
  suite(`Plugin: ${name}`, () => tests.forEach(({ desc, msgs, state, result }) => {
    test(desc, async () => {
      mockBackendState([]);
      let curState = state;
      for (const msg of msgs) {
        const result = await UpdateFunction(msg, plugins, curState);
        expect(result._tag).toBe("Right");
        if (E.isLeft(result)) {
          throw result.left;
        } else {
          curState = U.ModelOverwrite(curState, result.right);
        }
      }
      expect(curState).toStrictEqual(result);
    });
  }));
});



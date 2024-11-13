import { afterEach, expect, suite, test } from 'vitest';
import { clearMocks } from '@tauri-apps/api/mocks';
import {
  TMsg,
  TMap,
  NmluginVerified as Nmlugin,
  tInt,
  ModelOverwrite,
} from "@nmide/js-utils";
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
          Msg: ["increment", tInt(0)]
        },
        state: [["counter", tInt(0)]],
        result: [["counter", tInt(0)]],
      },
      {
        desc: "Non-increment messages should be ignore, and have no state change",
        msg: {
          Msg: ["not-increment", tInt(1)]
        },
        state: [["counter", tInt(0)]],
        result: [],
      },
      {
        desc: "Incrementing with 1, should change the state",
        msg: {
          Msg: ["increment", tInt(1)]
        },
        state: [["counter", tInt(0)]],
        result: [["counter", tInt(1)]],
      },
    ]
  }
];


const mockBackendState =
  (response: [string, TMap][]) => NmideClientMock("update", response);

singleMsgStateChangeTestData.forEach(({ name, plugins, tests }) => {
  suite(
    `Plugin: ${name}`,
    () => tests.forEach(({ desc, msg, state, result }) => {
      test(desc, async () => {
        mockBackendState([]);
        expect(await UpdateFunction(msg, plugins, state)).toStrictEqual(E.right([result, []]));
      });
    })
  );
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
          { Msg: ["increment", tInt(0)] },
          { Msg: ["increment", tInt(0)] },
          { Msg: ["increment", tInt(0)] },
          { Msg: ["increment", tInt(0)] },
          { Msg: ["increment", tInt(0)] },
          { Msg: ["increment", tInt(0)] },
        ],
        state: [["counter", tInt(10)]],
        result: [["counter", tInt(10)]],
      },
      {
        desc: "Non-increment messages should be ignore, and have no state change",
        msgs: [
          { Msg: ["not-increment", tInt(1)] },
          { Msg: ["incrmnt", tInt(100)] },
          { Msg: ["foo", tInt(3289)] },
          { Msg: ["foobar", tInt(332)] },
          { Msg: ["baz", tInt(10010)] },
          { Msg: ["!!!jc", tInt(31)] },
          { Msg: ["[Object object]", tInt(12020202020)] },
        ],
        state: [["counter", tInt(0)]],
        result: [["counter", tInt(0)]],
      },
      {
        desc: "Incrementing with 1, should change the state",
        msgs: [
          { Msg: ["increment", tInt(1)] },
          { Msg: ["increment", tInt(2)] },
          { Msg: ["increment", tInt(3)] },
          { Msg: ["increment", tInt(4)] },
          { Msg: ["increment", tInt(5)] },
          { Msg: ["increment", tInt(6)] },
          { Msg: ["increment", tInt(7)] },
          { Msg: ["increment", tInt(8)] },
          { Msg: ["increment", tInt(9)] },
        ],
        state: [["counter", tInt(0)]],
        result: [["counter", tInt(45)]],
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
          curState = ModelOverwrite(curState, result.right[0]);
        }
      }
      expect(curState, "Final state should be equal to the expected result")
        .toStrictEqual(result);
    });
  }));
});



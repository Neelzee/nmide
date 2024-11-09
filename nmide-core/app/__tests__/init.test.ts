import { afterEach, expect, test } from 'vitest';
import { InitFunction } from "../lib/Init";
import { clearMocks, mockIPC } from '@tauri-apps/api/mocks';
import * as E from "fp-ts/Either";
import TrivialPlugin from './test_plugins/trivial_plugin';
import { TMap } from '../lib/bindings/TMap';
import CounterPlugin from './test_plugins/counter_plugin';
import { NmideArgs, NmideDecodedType, NmideDecoderTest } from '../lib/NmideClient';

afterEach(clearMocks);

export const NmideClientMock = <
  K extends keyof NmideArgs & keyof typeof NmideDecoderTest,
>(
  _: K,
  response: NmideDecodedType<K>,
) => mockIPC((_, __?) => new Promise<any>(r => r(response)));

const mockState = (state: [string, TMap][]) => NmideClientMock("init", state);

test("No plugins? No state? No problem!", async () => {
  mockState([]);
  expect(await InitFunction([])).toStrictEqual(E.right([]));
});

test("Empty Plugin", async () => {
  const model: TMap = [["FooBar", { Int: 1 }]];
  const backendResponse: [string, TMap][] = [["State", model]];
  mockState(backendResponse);
  expect(await InitFunction([["TrivialPlugin", TrivialPlugin]]))
    .toStrictEqual(E.right(model));
});

test("Counter Plugin, works as expected", async () => {
  mockState([]);
  expect(await InitFunction([["CounterPlugin", CounterPlugin]]))
    .toStrictEqual(E.right([["counter", { Int: 0 }]]));
});


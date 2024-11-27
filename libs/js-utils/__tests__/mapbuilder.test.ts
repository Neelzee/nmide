import { expect, test } from 'vitest';
import MapBuilder from '../lib/MapBuilder';
import { tFloat, tInt, tList, tObj, tStr, tValueMaybe, TValuePrimitive } from '../lib/Types';
import { TMap } from '../lib/TMap';
import { tLookup } from '../lib/Utils';

test("Empty TMap", () => {
  expect(new MapBuilder().build()).toStrictEqual([]);
});

test("1-level TMap", () => {
  const data: TValuePrimitive[] = [
    1,
    "foobar",
    -1,
    1.2,
    [["obj_1", 1], ["obj_2", 2]],
    [1, 2, 3, 4, 5],
  ];
  const results: TMap[] = [
    [["field", tInt(1)]],
    [["field", tStr("foobar")]],
    [["field", tInt(-1)]],
    [["field", tFloat(1.2)]],
    [["field", tObj([["obj_1", 1], ["obj_2", 2]])]],
    [["field", tList([1, 2, 3, 4, 5])]]
  ];
  data.forEach(
    (input, index) => {
      const output = new MapBuilder().add("field", input).build();
      expect(
        output,
        `input: ${JSON.stringify(input)}\noutput: ${JSON.stringify(results[index])}`
      ).toStrictEqual(results[index]);
    }
  );
});

import { expect, suite, test } from 'vitest';
import { TMapPartialEq } from '../lib/Eq';
import MapBuilder from '../lib/MapBuilder';

test("Collisions", () => {
  const map1 = new MapBuilder()
    .add("bar", "bar")
    .add("foo", "foo")
    .build();
  const map2 = new MapBuilder().add("foo", "foo").build();
  expect(TMapPartialEq.equals(map1, map2)).toBe(true);
});

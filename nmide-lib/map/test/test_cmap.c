#include "test_cmap.h"

void test_adding_keys() {
  CMap *map = create_cmap();

  printf("\nEmpty map should have 0 values: %zu\n", map->val_len);
  munit_assert_size(map->val_len, ==, 0);

  CKey *key = new_ckey();

  change_key(key, "foo");

  CVal *val = new_val(Str, "bar");

  cmap_insert(map, val, key);

  printf("\nAfter insert, map should have 1 value: %zu\n", map->val_len);
  munit_assert_size(map->val_len, ==, 1);

  free_map(map);
}

void test_lookup() {
  CMap *map = create_cmap();

  CKey *key = new_ckey();

  change_key(key, "foo");
  printf("\nEmpty map should have 0 keys");

  MaybeVal *tuple = cmap_lookup(map, key);
  munit_assert_false(tuple->just);

  CVal *val = new_val(Str, "bar");

  cmap_insert(map, val, key);

  printf("\nAfter insert, map should have 1 value: %zu\n", map->val_len);
  munit_assert_size(map->val_len, ==, 1);

  MaybeVal *tuple_2 = cmap_lookup(map, key);

  printf("\nMap should have 1 key called foo: %d\n", tuple_2->just);
  munit_assert_true(tuple_2->just);
}

void test_removing_keys() {
  // Setup
  CMap *map = create_cmap();

  CKey *key = new_ckey();

  change_key(key, "foo");

  // Testing

  MaybeVal *res = cmap_remove(map, key);

  printf("\nRemoval on an empty map should not change len: %zu\n",
         map->val_len);
  munit_assert_size(map->val_len, ==, 0);
  printf("\nThe maybe value should not be `just`: %d\n", res->just);
  munit_assert_false(res->just);

  // Setup

  CVal *val = new_val(Str, "bar");

  cmap_insert(map, val, key);

  // Testing

  cmap_remove(map, key);

  printf("\nAfter remove, map should have 0 value: %zu\n", map->val_len);
  munit_assert_size(map->val_len, ==, 0);

  free_map(map);
}

void test_cmap() {
  run_test(test_adding_keys, "test_adding_keys",
           "tests if adding keys to the cmap increases the key count");

  run_test(test_lookup, "test_lookup", "tests if lookup works");

  run_test(test_removing_keys, "test_removing_keys",
           "tests if removing keys from the cmap changes the key count");
}

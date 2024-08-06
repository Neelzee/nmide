#include "test_cmap.h"

MunitResult test_adding_keys(const MunitParameter _[], void *__) {
  CMap *map = create_cmap();

  munit_assert_size(map->val_len, ==, 0);

  CKey *key = new_ckey();

  change_key(key, "foo");

  CVal *val = new_val(Str, "bar");

  cmap_insert(map, val, key);

  munit_assert_size(map->val_len, ==, 1);

  free_map(map);

  return MUNIT_OK;
}

MunitResult test_lookup(const MunitParameter _[], void *__) {
  CMap *map = create_cmap();

  CKey *key = new_ckey();

  change_key(key, "foo");

  MaybeVal *tuple = cmap_lookup(map, key);
  munit_assert_false(tuple->just);

  CVal *val = new_val(Str, "bar");

  cmap_insert(map, val, key);

  munit_assert_size(map->val_len, ==, 1);

  MaybeVal *tuple_2 = cmap_lookup(map, key);

  munit_assert_true(tuple_2->just);

  return MUNIT_OK;
}

MunitResult test_removing_keys(const MunitParameter _[], void *__) {
  // Setup
  CMap *map = create_cmap();

  CKey *key = new_ckey();

  change_key(key, "foo");

  // Testing

  MaybeVal *res = cmap_remove(map, key);

  munit_assert_size(map->val_len, ==, 0);
  munit_assert_false(res->just);

  // Setup

  CVal *val = new_val(Str, "bar");

  cmap_insert(map, val, key);

  // Testing

  cmap_remove(map, key);

  munit_assert_size(map->val_len, ==, 0);

  free_map(map);

  return MUNIT_OK;
}

MunitTest cmap_tests[] = {
    {
        (char *)"/test_adding_keys", /* name */
        test_adding_keys,            /* test */
        NULL,                        /* setup */
        NULL,                        /* tear_down */
        MUNIT_TEST_OPTION_NONE,      /* options */
        NULL                         /* parameters */
    },
    {
        (char *)"/test_lookup", /* name */
        test_lookup,            /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        NULL                    /* parameters */
    },
    {
        (char *)"/test_removing_keys", /* name */
        test_removing_keys,            /* test */
        NULL,                          /* setup */
        NULL,                          /* tear_down */
        MUNIT_TEST_OPTION_NONE,        /* options */
        NULL                           /* parameters */
    },
    /* Mark the end of the array with an entry where the test
     * function is NULL */
    {NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL},
};

const MunitSuite suite = {(char *)"/cmap_tests", cmap_tests, NULL, 1,
                          MUNIT_SUITE_OPTION_NONE};

int main(int argc, char *argv[]) {
  return munit_suite_main(&suite, (void *)"munit", argc, argv);
}

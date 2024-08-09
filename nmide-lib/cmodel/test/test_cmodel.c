#include "test_cmodel.h"

MunitResult test_init(const MunitParameter _[], void *__) {
  CModel *model = cmodel_init();
  munit_assert_not_null(model);
  MaybeVal *loc = cmodel_lookup(model, "location");
  munit_assert_true(loc->just);
  munit_assert_not_null(loc->val);
  munit_assert_null(carr_get(loc->val->val->arr, 0));
  drop(model);
  free_maybe(loc);
  return MUNIT_OK;
}

MunitResult test_insert(const MunitParameter _[], void *__) {
  CModel *model = cmodel_init();
  munit_assert_false(cmodel_insert(model, new_val(Str, "foobar"), "foo"));
  munit_assert_true(cmodel_insert(model, new_val(Str, "foobaz"), "foo"));
  drop(model);
  return MUNIT_OK;
}

MunitResult test_lookup(const MunitParameter _[], void *__) {
  CModel *model = cmodel_init();
  cmodel_insert(model, new_val(Str, "foobar"), "foo");
  munit_assert_true(cmodel_lookup(model, "foobar")->just);
  drop(model);
  return MUNIT_OK;
}

MunitResult test_removing_keys(const MunitParameter _[], void *__) {
  CModel *model = cmodel_init();
  cmodel_insert(model, new_val(Str, "foo"), "foo");
  cmodel_insert(model, new_val(Str, "bar"), "foo");
  munit_assert_true(cmodel_remove(model, "foo")->just);
  munit_assert_false(cmodel_remove(model, "foo")->just);
  munit_assert_true(cmodel_remove(model, "bar")->just);
  munit_assert_false(cmodel_remove(model, "baz")->just);
  drop(model);
  return MUNIT_OK;
}

MunitTest tests[] = {
    {
        (char *)"/init",        /* name */
        test_init,              /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        NULL                    /* parameters */
    },
    {
        (char *)"/insert",      /* name */
        test_init,              /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        NULL                    /* parameters */
    },

    {
        (char *)"/lookup",      /* name */
        test_lookup,            /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        NULL                    /* parameters */
    },
    {
        (char *)"/remove",      /* name */
        test_removing_keys,     /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        NULL                    /* parameters */
    },
    /* Mark the end of the array with an entry where the test
     * function is NULL */
    {NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL},
};

const MunitSuite suite = {(char *)"/cmodel", tests, NULL, 1,
                          MUNIT_SUITE_OPTION_NONE};

int main(int argc, char *argv[]) {
  return munit_suite_main(&suite, NULL, argc, argv);
}

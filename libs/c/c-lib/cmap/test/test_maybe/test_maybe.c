#include "test_maybe.h"

// Maybe Test Suite ============================================================

static MunitResult test_maybe_nothing(const MunitParameter _[], void *__) {
  MaybeVal *m = maybe(NULL);
  munit_assert_not_null(m);
  munit_assert_false(m->just);
  free_maybe(m);
  return MUNIT_OK;
}

static MunitResult test_maybe(const MunitParameter _[], void *__) {
  MaybeVal *m = maybe(new_val(Str, "foobar"));
  munit_assert_not_null(m);
  munit_assert(m->just);
  free_maybe(m);
  return MUNIT_OK;
}

static MunitTest maybe_tests[] = {
    {
        (char *)"/nothing",     /* name */
        test_maybe_nothing,     /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        NULL                    /* parameters */
    },
    {
        (char *)"/just",        /* name */
        test_maybe,             /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        NULL                    /* parameters */
    },
    /* Mark the end of the array with an entry where the test
     * function is NULL */
    {NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL},
};

// =============================================================================

// Suite Declaration ===========================================================

const MunitSuite maybe_suite = {
    (char *)"/maybe",       // suite name
    maybe_tests,            // tests
    NULL,                   // sub-suites
    1,                      // iterations
    MUNIT_SUITE_OPTION_NONE // options
};

// =============================================================================

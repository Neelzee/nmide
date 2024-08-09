#include "test_ckey.h"

// CKey Test Suite =============================================================

static MunitResult test_key(const MunitParameter _[], void *__) {
  CKey *k = key("");
  munit_assert_not_null(k);
  free_key(k);
  return MUNIT_OK;
}

static MunitTest ckey_tests[] = {
    {
        (char *)"/new",         /* name */
        test_key,               /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        NULL                    /* parameters */
    },
    /* Mark the end of the array with an entry where the test
     * function is NULL */
    {NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL},
};

// Suite Declaration ===========================================================

const MunitSuite ckey_suite = {
    (char *)"/ckey",        // suite name
    ckey_tests,             // "super-suite"
    NULL,                   // sub-suites
    1,                      // iterations
    MUNIT_SUITE_OPTION_NONE // options
};

// =============================================================================

#include "test_cval.h"

// CVal Test Suite =============================================================

static MunitResult test_new_val(const MunitParameter _[], void *__) {
  CVal *val = new_val(Str, "val");
  munit_assert_not_null(val);
  free_cval(val);
  return MUNIT_OK;
}

static MunitTest cval_tests[] = {
    {
        (char *)"/new",         /* name */
        test_new_val,           /* test */
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

const MunitSuite cval_suite = {
    (char *)"/cval",        // suite name
    cval_tests,             // "super-suite"
    NULL,                   // sub-suites
    1,                      // iterations
    MUNIT_SUITE_OPTION_NONE // options
};

// =============================================================================

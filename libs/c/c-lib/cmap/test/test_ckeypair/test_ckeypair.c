#include "test_ckeypair.h"

// CKeyPair Test Suite =========================================================

static MunitResult test_key_pair(const MunitParameter _[], void *__) {
  CKeyPair *pair = key_pair("foo", new_val(Str, "bar"));
  munit_assert_not_null(pair);
  free_keypair(pair);
  return MUNIT_OK;
}

static MunitTest ckeypair_tests[] = {
    {
        (char *)"/new",         /* name */
        test_key_pair,          /* test */
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

const MunitSuite ckeypair_suite = {
    (char *)"/ckeypair",    // suite name
    ckeypair_tests,         // "super-suite"
    NULL,                   // sub-suites
    1,                      // iterations
    MUNIT_SUITE_OPTION_NONE // options
};

// =============================================================================

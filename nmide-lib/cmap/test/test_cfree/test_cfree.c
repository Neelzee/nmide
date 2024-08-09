#include "test_cfree.h"

// Free Test Suite =============================================================

static MunitResult test_free_cval(const MunitParameter param[], void *__) {
  CVal *val = new_val(Str, (void *)"str");
  munit_assert_not_null(val);
  free_cval(val);
  munit_assert_null(val);
  return MUNIT_OK;
}

static MunitResult test_free_maybe(const MunitParameter _[], void *__) {
  MaybeVal *val = maybe(new_val(Str, (void *)"str"));
  munit_assert_not_null(val);
  free_maybe(val);
  munit_assert_null(val);
  return MUNIT_OK;
}

static MunitResult test_free_key(const MunitParameter _[], void *__) {
  CKey *k = key("foo");
  munit_assert_not_null(k);
  free_key(k);
  munit_assert_null(k);
  return MUNIT_OK;
}

static MunitResult test_free_keypair(const MunitParameter _[], void *__) {
  CKeyPair *k = key_pair("foo", new_val(Str, (void *)"str"));
  munit_assert_not_null(k);
  free_keypair(k);
  munit_assert_null(k);
  return MUNIT_OK;
}

static MunitResult test_free_arr(const MunitParameter _[], void *__) {
  CArr *arr = new_arr(NULL);
  munit_assert_not_null(arr);
  free_arr(arr);
  munit_assert_null(arr);
  return MUNIT_OK;
}

static MunitResult test_free_map(const MunitParameter _[], void *__) {
  CMap *map = create_cmap();
  munit_assert_not_null(map);
  free_map(map);
  munit_assert_null(map);
  return MUNIT_OK;
}

static MunitTest free_tests[] = {
    {
        (char *)"/cval",        /* name */
        test_free_cval,         /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        NULL                    /* parameters */
    },
    {
        (char *)"/maybe",       /* name */
        test_free_maybe,        /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        NULL                    /* parameters */
    },
    {
        (char *)"/ckey",        /* name */
        test_free_key,          /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        NULL                    /* parameters */
    },
    {
        (char *)"/ckeypair",    /* name */
        test_free_keypair,      /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        NULL                    /* parameters */
    },
    {
        (char *)"/carr",        /* name */
        test_free_arr,          /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        NULL                    /* parameters */
    },
    {
        (char *)"/cmap",        /* name */
        test_free_map,          /* test */
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

const MunitSuite cfree_suite = {
    (char *)"/free",        // suite name
    free_tests,             // "super-suite"
    NULL,                   // sub-suites
    1,                      // iterations
    MUNIT_SUITE_OPTION_NONE // options
};

// =============================================================================

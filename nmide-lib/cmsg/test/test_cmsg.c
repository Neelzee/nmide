#include "test_cmsg.h"

// CMsg Helpers ================================================================

static CVal *get_cval_from_str(char *val, char *type) {
  CVal *v = (CVal *)malloc(sizeof(CVal));
  if (strcmp(type, "Int")) {
    v->type = Int;
    v->val._int = atoi(val);
  } else {
    v->type = Str;
    v->val.str = val;
  }
  return v;
}

// =============================================================================

// CMsg Tests ==================================================================

/**
 * Creating a CMsg should always work.
 */
static MunitResult test_new_cmsg(const MunitParameter params[], void *__) {
  char *msg = (char *)munit_parameters_get(params, "msg_b");
  CMsg *m = new_cmsg(msg, NULL);
  munit_assert_not_null(m);
  return MUNIT_OK;
}

/**
 * Dropping/freeing a CMsg should not leak any memory
 */
static MunitResult test_drop_cmsg(const MunitParameter params[],
                                  void *user_data) {
  CMsg *msg_a = (CMsg *)user_data;
  drop_cmsg(msg_a);
  return MUNIT_OK;
}

/**
 * Comparrision on CMsg should work regardless of value, and be true if their
 * msg are the same. Comparrision should also be case-insensitive
 *
 * # Test Scenario:
 *
 * - Two Different strings - false
 * - Two of the same strings - true
 * - Two of the same strings, different cases - true
 */
static MunitResult test_cmsg_kind_cmp(const MunitParameter params[], void *_) {
  CMsg *msg_a = new_cmsg((char *)munit_parameters_get(params, "msg_a"), NULL);
  char *msg_b_str = (char *)munit_parameters_get(params, "msg_b");
  CMsg *msg_b = new_cmsg((char *)munit_parameters_get(params, "msg_a"), NULL);
  bool result = cmsg_kind_cmp(msg_a, msg_b);
  bool expected = strcmp(msg_a->msg, msg_b->msg);
  if (expected) {
    munit_assert_true(result);
  } else {
    munit_assert_false(result);
  }
  return MUNIT_OK;
}

// =============================================================================

// CMsg Params =================================================================

static char *msgs[] = {(char *)"oof",
                       (char *)"æøå",
                       (char *)"ÆÆÆÆ",
                       (char *)"!!?#)?=!#",
                       (char *)"-10",
                       (char *)"ABS",
                       (char *)"OOF",
                       (char *)"RABOOF",
                       (char *)"abs",
                       (char *)"10",
                       NULL};

static MunitParameterEnum params[] = {
    {(char *)"msg_a", msgs}, {(char *)"msg_b", msgs}, {NULL, NULL}};

// =============================================================================

static MunitTest tests[] = {
    {
        (char *)"/new",         /* name */
        test_new_cmsg,          /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        params                  /* parameters */
    },
    {
        (char *)"/drop",        /* name */
        test_drop_cmsg,         /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        params                  /* parameters */
    },
    {
        (char *)"/kind_cmp",    /* name */
        test_cmsg_kind_cmp,     /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        params                  /* parameters */
    },
    /* Mark the end of the array with an entry where the test
     * function is NULL */
    {NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL},
};

static const MunitSuite suite = {(char *)"/cmsg", tests, NULL, 1,
                                 MUNIT_SUITE_OPTION_NONE};

int main(int argc, char *argv[]) {
  return munit_suite_main(&suite, NULL, argc, argv);
}

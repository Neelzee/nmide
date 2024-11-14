#include "test_cmap.h"

static CValType get_type_from_str(char *str) {
  if (strcmp(str, "Int")) {
    return Int;
  } else {
    return Str;
  }
}

static void get_val_from_str(CVal *val, char *str) {
  switch (val->type) {
  case Str:
    val->val.str = str;
  case Int:
    val->val._int = atoi(str);
  case Arr:
  case Obj:
    break;
  }
}

// CMap Test Suite =============================================================

static MunitResult test_create_cmap(const MunitParameter _[], void *__) {
  CMap *map = create_cmap();
  munit_assert_not_null(map);
  free_map(map);
  return MUNIT_OK;
}

/*
 * Different types we insert into the CMap.
 * Works with both Str and Int, because atoi is total on all strings
 */
static char *add_types[] = {(char *)"Str", (char *)"Int", NULL};

// Corresponding values
static char *add_val[] = {(char *)"oof",    (char *)"rab", (char *)"zab",
                          (char *)"raboof", (char *)"10",  NULL};

static char *add_key[] = {(char *)"oof",    (char *)"rab", (char *)"zab",
                          (char *)"raboof", (char *)"10",  NULL};

static char *remove_key[] = {(char *)"oof",    (char *)"rab", (char *)"zab",
                             (char *)"raboof", (char *)"10",  NULL};

static MunitParameterEnum insert_params[] = {{(char *)"type", add_types},
                                             {(char *)"val", add_val},
                                             {(char *)"key", add_val},
                                             {(char *)"remove", remove_key},
                                             {NULL, NULL}};

static void *cmap_setup(const MunitParameter params[], void *user_data) {
  return create_cmap();
}

static void cmap_tear_down(void *fixture) { free_map((CMap *)fixture); }

static MunitResult test_cmap_insert(const MunitParameter params[],
                                    void *user_data) {
  return MUNIT_FAIL;
}

static MunitResult test_cmap_lookup(const MunitParameter _[], void *__) {
  return MUNIT_FAIL;
}

static MunitResult test_cmap_remove(const MunitParameter params[],
                                    void *user_data) {
  return MUNIT_FAIL;
}

static MunitTest cmap_tests[] = {
    {
        (char *)"/create",      /* name */
        test_create_cmap,       /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        NULL                    /* parameters */
    },
    {
        (char *)"/insert",      /* name */
        test_cmap_insert,       /* test */
        cmap_setup,             /* setup */
        cmap_tear_down,         /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        insert_params           /* parameters */
    },
    {
        (char *)"/lookup",      /* name */
        test_cmap_lookup,       /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        NULL                    /* parameters */
    },
    {
        (char *)"/remove",      /* name */
        test_cmap_remove,       /* test */
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

// Map Size Test Suite =========================================================

static MunitResult test_map_size_empty(const MunitParameter _[], void *__) {
  CMap *map = create_cmap();
  munit_assert_null(map);
  if (map == NULL) {
    return MUNIT_FAIL;
  }
  size_t result = map->len;
  size_t expected = 0;
  munit_assert_int(result, ==, expected);
  return MUNIT_OK;
}

static char *map_size_key_name_params[] = {
    (char *)"foo", (char *)"bar", (char *)"baz", (char *)"foobar", NULL};

static char *map_size_value_params[] = {(char *)"oof", (char *)"rab",
                                        (char *)"zab", (char *)"raboof", NULL};
static MunitParameterEnum test_add_elem_params[] = {
    {(char *)"key", map_size_key_name_params},
    {(char *)"val", map_size_value_params},
    {NULL, NULL}};

static MunitResult test_map_size_add_elem(const MunitParameter params[],
                                          void *__) {
  // TODO
  return MUNIT_FAIL;
}

static MunitResult test_map_size_add_elem_remove_elem(const MunitParameter _[],
                                                      void *__) {
  // TODO
  return MUNIT_FAIL;
}

static MunitTest cmap_size_tests[] = {
    {
        (char *)"/empty",       /* name */
        test_map_size_empty,    /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        NULL                    /* parameters */
    },
    {
        (char *)"/add_elem",    /* name */
        test_map_size_add_elem, /* test */
        NULL,                   /* setup */
        NULL,                   /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        test_add_elem_params    /* parameters */
    },
    {
        (char *)"/add_elem_remove_elem",    /* name */
        test_map_size_add_elem_remove_elem, /* test */
        NULL,                               /* setup */
        NULL,                               /* tear_down */
        MUNIT_TEST_OPTION_NONE,             /* options */
        NULL                                /* parameters */
    },
    /* Mark the end of the array with an entry where the test
     * function is NULL */
    {NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL},
};

static const MunitSuite cmap_size_suite = {
    (char *)"/size",        // suite name
    cmap_size_tests,        // "super-suite"
    NULL,                   // sub-suites
    1,                      // iterations
    MUNIT_SUITE_OPTION_NONE // options
};

// =============================================================================

// Suite Declaration ===========================================================

static MunitSuite sub_suites[] = {cmap_size_suite, NULL};

const MunitSuite cmap_suite = {
    (char *)"/cmap",        // suite name
    cmap_tests,             // "super-suite"
    NULL,                   // sub-suites
    1,                      // iterations
    MUNIT_SUITE_OPTION_NONE // options
};

// =============================================================================

#include "test_carr.h"

// Arr Helpers =================================================================

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
    val->val->str = str;
  case Int:
    val->val->_int = atoi(str);
  case Arr:
  case Obj:
    break;
  }
}

// Arr Size Test Suite =========================================================

static void *arr_setup(const MunitParameter params[], void *user_data) {
  CArr *arr = (CArr *)malloc(sizeof(CArr));       // allocates memory
  CVal **elems = (CVal **)malloc(sizeof(CVal *)); // allocates memory to elems
  elems[0] = NULL;       // its an empty CArr, so the only elem is NULL
  arr->elements = elems; // points CArr to elems
  return arr;            // returns CArr
}

static void arr_tear_down(void *fixture) {
  int i = 0;                     // sets counter to 0
  CArr *arr = (CArr *)fixture;   // casts fixture to arr
  CVal *elem = arr->elements[i]; // gets the 0th elem
  while (elem != NULL) {         // since last elem is NULL
    i++;                         // this will iterate over all elems
    free(elem->val);             // val is CValUnion, and needs to be free'd
    free(elem);                  // now we free CVal
    elem = arr->elements[i];     // And get the next elem, until we are done
  }
  free(arr); // Finally, we can free arr
}

/**
 * Tests that an empty array has size 0
 */
static MunitResult test_arr_size_empty(const MunitParameter _[],
                                       void *user_data) {
  CArr *arr = (CArr *)user_data;
  size_t size = arr_size(arr);
  munit_assert_int(size, ==, 0);
  return MUNIT_OK;
}

/*
 * Different types we insert into the CArr.
 * Works with both Str and Int, because atoi is total on all strings
 */
static char *arr_add_types[] = {(char *)"Str", (char *)"Int", NULL};

// Corresponding values
static char *arr_add_val[] = {(char *)"oof",    (char *)"rab", (char *)"zab",
                              (char *)"raboof", (char *)"10",  NULL};

static MunitParameterEnum arr_add_params[] = {{(char *)"type", arr_add_types},
                                              {(char *)"val", arr_add_val},
                                              {NULL, NULL}};

static MunitResult test_arr_size_add_elem(const MunitParameter params[],
                                          void *user_data) {
  CArr *arr = (CArr *)user_data; // Gets init. CArr
  char *type_str = (char *)munit_parameters_get(params, "type"); // type
  char *val_str = (char *)munit_parameters_get(params, "val");   // value
  int count = munit_rand_int_range(0, 200);                      // random count
  for (int i = 0; i < count; i++) {
    CVal *val = (CVal *)malloc(sizeof(CVal));          // Allocate CVal
    val->val = (CValUnion *)malloc(sizeof(CValUnion)); // Allocate CValUnion
    val->type = get_type_from_str(type_str); // memory is free'd in teardown
    get_val_from_str(val, val_str);
    insert_arr(arr, val); // Inserts the elem
  }
  size_t size = arr_size(arr);
  munit_assert_int(size, ==,
                   count); // The size should how many elements we added
  return MUNIT_OK;
}

static MunitResult
test_arr_size_add_elem_remove_elem(const MunitParameter params[],
                                   void *user_data) {
  CArr *arr = (CArr *)user_data;
  char *type_str = (char *)munit_parameters_get(params, "type");
  char *val_str = (char *)munit_parameters_get(params, "val");
  int add = munit_rand_int_range(0, 200);    // random count
  int remove = munit_rand_int_range(0, add); // Remove a random amount of elems
  for (int i = 0; i < add; i++) {
    CVal *val = (CVal *)malloc(sizeof(CVal));
    val->val = (CValUnion *)malloc(sizeof(CValUnion));
    val->type = get_type_from_str(type_str);
    get_val_from_str(val, val_str);
    insert_arr(arr, val);
  }
  for (int i = 0; i < remove; i++) {
    carr_remove(arr, 0);
  }
  int count = add - remove;
  count = count < 0 ? 0 : count;
  size_t size = arr_size(arr);
  munit_assert_int(size, ==, count);
  return MUNIT_OK;
}

static MunitTest carr_size_tests[] = {
    {
        (char *)"/empty",       /* name */
        test_arr_size_empty,    /* test */
        arr_setup,              /* setup */
        arr_tear_down,          /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        NULL                    /* parameters */
    },
    {
        (char *)"/add_elem",    /* name */
        test_arr_size_add_elem, /* test */
        arr_setup,              /* setup */
        arr_tear_down,          /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        arr_add_params          /* parameters */
    },
    {
        (char *)"/add_elem_remove_elem",    /* name */
        test_arr_size_add_elem_remove_elem, /* test */
        arr_setup,                          /* setup */
        arr_tear_down,                      /* tear_down */
        MUNIT_TEST_OPTION_NONE,             /* options */
        arr_add_params                      /* parameters */
    },
    /* Mark the end of the array with an entry where the test
     * function is NULL */
    {NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL},
};

static const MunitSuite size_suite = {
    (char *)"/size",        // suite name
    carr_size_tests,        // tests
    NULL,                   // sub-suites
    3,                      // iterations
    MUNIT_SUITE_OPTION_NONE // options
};

// =============================================================================

// Arr Get Test Suite ==========================================================

static MunitResult test_arr_get_empty(const MunitParameter params[],
                                      void *user_data) {
  CArr *arr = (CArr *)user_data;
  size_t index = munit_rand_int_range(0, 200); // get at random index
  CVal *val = carr_get(arr, index);
  munit_assert_null(val);
  return MUNIT_OK;
}

static MunitResult test_arr_get_add_elem(const MunitParameter params[],
                                         void *user_data) {
  CArr *arr = (CArr *)user_data; // Gets init. CArr
  char *type_str = (char *)munit_parameters_get(params, "type"); // type
  char *val_str = (char *)munit_parameters_get(params, "val");   // value
  int count = munit_rand_int_range(0, 200);                      // random count
  size_t index = munit_rand_int_range(0, 200); // get at random index
  for (int i = 0; i < count; i++) {
    CVal *val = (CVal *)malloc(sizeof(CVal));          // Allocate CVal
    val->val = (CValUnion *)malloc(sizeof(CValUnion)); // Allocate CValUnion
    val->type = get_type_from_str(type_str); // memory is free'd in teardown
    get_val_from_str(val, val_str);
    insert_arr(arr, val); // Inserts the elem
  }
  CVal *val = carr_get(arr, index);
  if (index >= count) { // Of index is out of bounds, it should be null
    munit_assert_null(val);
  } else { // else, it should not be null
    munit_assert_not_null(val);
  }
  return MUNIT_OK;
}

static MunitResult
test_arr_get_add_elem_remove_elem(const MunitParameter params[],
                                  void *user_data) {
  CArr *arr = (CArr *)user_data;
  char *type_str = (char *)munit_parameters_get(params, "type");
  char *val_str = (char *)munit_parameters_get(params, "val");
  int add = munit_rand_int_range(0, 200);    // random count
  int remove = munit_rand_int_range(0, add); // Remove a random amount of elems
  size_t index = munit_rand_int_range(0, 200); // get at random index
  for (int i = 0; i < add; i++) {
    CVal *val = (CVal *)malloc(sizeof(CVal));
    val->val = (CValUnion *)malloc(sizeof(CValUnion));
    val->type = get_type_from_str(type_str);
    get_val_from_str(val, val_str);
    insert_arr(arr, val);
  }
  for (int i = 0; i < remove; i++) {
    CVal *res = carr_remove(arr, 0);
    munit_assert_not_null(res);
  }
  CVal *val = carr_get(arr, index);
  int count = add - remove;
  count = count < 0 ? 0 : count;
  if (index >= count) { // Of index is out of bounds, it should be null
    munit_assert_null(val);
  } else { // else, it should not be null
    munit_assert_not_null(val);
  }
  munit_assert_null(val);
  return MUNIT_OK;
}

static MunitTest carr_get_tests[] = {
    {
        (char *)"/empty",       /* name */
        test_arr_get_empty,     /* test */
        arr_setup,              /* setup */
        arr_tear_down,          /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        NULL                    /* parameters */
    },
    {
        (char *)"/add_elem",    /* name */
        test_arr_get_add_elem,  /* test */
        arr_setup,              /* setup */
        arr_tear_down,          /* tear_down */
        MUNIT_TEST_OPTION_NONE, /* options */
        arr_add_params          /* parameters */
    },
    {
        (char *)"/add_elem_remove_elem",   /* name */
        test_arr_get_add_elem_remove_elem, /* test */
        arr_setup,                         /* setup */
        arr_tear_down,                     /* tear_down */
        MUNIT_TEST_OPTION_NONE,            /* options */
        arr_add_params                     /* parameters */
    },
    /* Mark the end of the array with an entry where the test
     * function is NULL */
    {NULL, NULL, NULL, NULL, MUNIT_TEST_OPTION_NONE, NULL},
};

static const MunitSuite get_suite = {
    (char *)"/get",         // suite name
    carr_get_tests,         // tests
    NULL,                   // sub-suites
    3,                      // iterations
    MUNIT_SUITE_OPTION_NONE // options
};

// =============================================================================

// Suite Declaration ===========================================================

static MunitSuite sub_suites[] = {size_suite, get_suite, NULL};

const MunitSuite carr_suite = {
    (char *)"/carr",        // suite name
    NULL,                   // tests
    sub_suites,             // sub-suites
    1,                      // iterations
    MUNIT_SUITE_OPTION_NONE // options
};

// =============================================================================

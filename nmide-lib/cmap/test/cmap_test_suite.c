#include "cmap_test_suite.h"

int main(int argc, char *argv[]) {
  MunitSuite sub_suites[] = {carr_suite, cfree_suite, ckeypair_suite,
                             cmap_suite, cval_suite,  maybe_suite,
                             NULL};

  const MunitSuite suite = {
      (char *)"/cmap_tests",  // suite name
      NULL,                   // "super-suite"
      sub_suites,             // sub-suites
      1,                      // iterations
      MUNIT_SUITE_OPTION_NONE // options
  };

  return munit_suite_main(&suite, NULL, argc, argv);
}

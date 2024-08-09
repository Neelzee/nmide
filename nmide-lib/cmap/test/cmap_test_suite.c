#include "cmap_test_suite.h"
#include "test_ckey/test_ckey.h"
#include "test_ckeypair/test_ckeypair.h"
#include "test_cmap/test_cmap.h"
#include "test_cval/test_cval.h"
#include "test_maybe/test_maybe.h"

int main(int argc, char *argv[]) {
  MunitSuite sub_suites[] = {carr_suite,     cfree_suite, ckey_suite,
                             ckeypair_suite, cmap_suite,  cval_suite,
                             maybe_suite,    NULL};

  const MunitSuite suite = {
      (char *)"/cmap_tests",  // suite name
      NULL,                   // "super-suite"
      sub_suites,             // sub-suites
      1,                      // iterations
      MUNIT_SUITE_OPTION_NONE // options
  };

  return munit_suite_main(&suite, NULL, argc, argv);
}

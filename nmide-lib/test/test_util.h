#ifndef TEST_UTIL

#define TEST_UTIL

#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

void run_test(void (*f)(), char *test_name, char *test_description);

char *repeatChar(char c, size_t count);

#endif // !TEST_UTIL

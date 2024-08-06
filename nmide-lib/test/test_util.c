#include "test_util.h"
#include <stdio.h>
#include <string.h>

const size_t bar_len = 80;
const size_t bar_start_const = 8;
const size_t bar_end_const = 6;
const char bar_symbol = '=';

void run_test(void (*f)(), char *test_name, char *test_description) {
  size_t non_bar = bar_start_const + strlen(test_name);
  size_t twice_bar = bar_len - non_bar;
  size_t left_bar = twice_bar / 2;
  size_t right_bar = twice_bar / 2;
  if (twice_bar % 2 == 1) {
    right_bar++;
  }
  char *left = repeatChar((char)bar_symbol, left_bar);
  char *right = repeatChar((char)bar_symbol, left_bar);
  printf("\n%s start-%s %s\n", left, test_name, right);
  printf("\nDescription:\n%s\n", test_description);
  printf("\n%s\n", repeatChar((char)bar_symbol, bar_len));
  free(left);
  free(right);
  f();
  {
    size_t non_bar = bar_end_const + strlen(test_name);
    size_t twice_bar = bar_len - non_bar;
    size_t left_bar = twice_bar / 2;
    size_t right_bar = twice_bar / 2;
    if (twice_bar % 2 == 1) {
      right_bar++;
    }
    char *end_left = repeatChar((char)bar_symbol, left_bar);
    char *end_right = repeatChar((char)bar_symbol, right_bar);
    printf("\n%s end-%s %s\n", end_left, test_name, end_right);
    free(end_left);
    free(end_right);
  }
}

char *repeatChar(char c, size_t count) {
  char *str = (char *)malloc((sizeof(char) * count) + 1);

  for (int i = 0; i < count; i++) {
    str[i] = c;
  }

  str[count] = '\0';

  return str;
}

char *zipTwoStr(char *str1, char *str2) {
  size_t s1 = strlen(str1);
  size_t s2 = strlen(str2);
  char *str3 = (char *)malloc((sizeof(char) * (s1 > s2 ? s2 : s1)) + 1);
  int j = 0;
  for (int i = 0; i < sizeof(str3); i++) {
    char c1 = str1[i];
    char c2 = str2[i];
    str3[j] = c1;
    str3[j + 1] = c2;
    j += 2;
  }
  str3[strlen(str3)] = '\0';
  return str3;
}

/**
 * repeatChar :: Char -> Int -> String
 * repeatChar _ 0 = ""
 * repeatChar c n = c : repeatChar c (n - 1)
 */

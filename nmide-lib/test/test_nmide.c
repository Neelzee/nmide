#include "test_nmide.h"
#include <stdio.h>

char *bar() { return "=============================="; }

void test_simple_test() {

  printf("%s simple_test_start %s\n", bar(), bar());

  CHtml *html;
  html = simple_test();

  // Outer most element should be a div, and therefore an element
  munit_assert_true(html->isElement);

  CHtmlElement *div = html->content->element;

  munit_assert_int(div->tag, ==, Div);

  munit_assert_size(div->len, ==, 1);

  CHtml *child = div->children[0];

  munit_assert_false(child->isElement);

  CHtmlText *text = child->content->text;

  printf("%s\n", text->text);

  printf("%s simple_test_end %s\n", bar(), bar());

  free_chtml(html);
}

int main(int argc, char *argv[]) {

  test_simple_test();

  return 0;
}

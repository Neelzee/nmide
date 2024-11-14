#include "chtml.h"

CHtmlElement *element(CHtmlTag tag) {
  CHtmlElement *e = (CHtmlElement *)malloc(sizeof(CHtmlElement));
  e->len = 0;
  e->tag = tag;

  return e;
}

CHtmlElement *e_div() { return element(Div); }

CHtmlContent *unionize(CHtmlElement *element, char *text) {
  CHtmlContent *e = (CHtmlContent *)malloc(sizeof(CHtmlContent));

  if (element == NULL && text == NULL) {
    fprintf(stderr, "Error: Both element and text is null");
    exit(1);
  }

  if (element != NULL && text != NULL) {
    fprintf(stderr, "Error: Both element and text are defined");
    exit(1);
  }

  if (element != NULL) {
    e->element = *element;
    free(element);
  }

  if (text != NULL) {
    e->text = text;
  }

  return e;
}

CHtml simple_test() {
  CHtmlElement *p_e_div = e_div();

  CHtmlContent *c_text = unionize(NULL, "Hello, world!");

  CHtml *html_text = (CHtml *)malloc(sizeof(CHtml));

  html_text->isElement = false;

  html_text->content = *c_text;
  free(c_text);

  p_e_div->children = (CHtml *)malloc(sizeof(CHtml));
  p_e_div->children[0] = *html_text;
  free(html_text);
  p_e_div->len++;

  CHtmlContent *c_div = unionize(p_e_div, NULL);

  CHtml *html_div = (CHtml *)malloc(sizeof(CHtml));

  html_div->isElement = true;
  html_div->content = *c_div;
  CHtml obj = *html_div;
  free_chtml(html_div);
  return obj;
}

void free_chtml(CHtml *chtml) {
  if (chtml != NULL) {

    if (chtml->isElement) {
      CHtmlElement element = chtml->content.element;
      free(element.children);
    }
    free(chtml);
  }
}

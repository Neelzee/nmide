#include "html.h"

CHtmlElement *element(CHtmlTag tag) {
  CHtmlElement *e = (CHtmlElement *)malloc(sizeof(CHtmlElement));
  e->len = 0;
  e->tag = tag;

  return e;
}

CHtmlElement *e_div() { return element(Div); }

CHtmlText *e_text() {
  CHtmlText *e = (CHtmlText *)malloc(sizeof(CHtmlText));
  e->text = "";
  e->len = 1;
  return e;
}

CHtmlContent *unionize(CHtmlElement *element, CHtmlText *text) {
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
    e->element = element;
  }

  if (text != NULL) {
    e->text = text;
  }

  return e;
}

CHtml *simple_test() {
  CHtmlElement *p_e_div = e_div();
  CHtmlText *p_e_text = e_text();

  p_e_text->text = "Hello, world!";
  p_e_text->len = sizeof(p_e_text->text);

  CHtmlContent *c_text = unionize(NULL, p_e_text);

  CHtml *html_text = (CHtml *)malloc(sizeof(CHtml));

  html_text->isElement = false;

  html_text->content = c_text;

  p_e_div->children = &html_text;
  p_e_div->len++;

  CHtmlContent *c_div = unionize(p_e_div, NULL);

  CHtml *html_div = (CHtml *)malloc(sizeof(CHtml));

  html_div->isElement = true;
  html_div->content = c_div;

  return html_div;
}

void free_chtml(CHtml *chtml) {
  if (chtml != NULL) {

    if (chtml->isElement) {
      CHtmlElement *element = chtml->content->element;

      while (element->len > 0) {
        CHtml *child = element->children[0];
        free_chtml(child);
        element->len--;
      }
    }
    free(chtml->content);
    free(chtml);
  }
}

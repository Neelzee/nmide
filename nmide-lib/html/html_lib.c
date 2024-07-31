#include "html_lib.h"

CHtml div() {
  CHtmlUnion u;
  u.kind = Div;

  CHtml html;
  html.kid_count = 0;
  html.node = u;
  html.isNode = 1;

  return html;
}

CHtml p() {
  CHtmlUnion u;
  u.kind = P;

  CHtml html;
  html.kid_count = 0;
  html.node = u;
  html.isNode = 1;

  return html;
}

CHtml span() {
  CHtmlUnion u;
  u.kind = Span;

  CHtml html;
  html.kid_count = 0;
  html.node = u;
  html.isNode = 1;

  return html;
}

CHtml section() {
  CHtmlUnion u;
  u.kind = Section;

  CHtml html;
  html.kid_count = 0;
  html.node = u;
  html.isNode = 1;

  return html;
}

CHtml input() {
  CHtmlUnion u;
  u.kind = Input;

  CHtml html;
  html.kid_count = 0;
  html.node = u;
  html.isNode = 1;

  return html;
}

CHtml button() {
  CHtmlUnion u;
  u.kind = Button;

  CHtml html;
  html.kid_count = 0;
  html.node = u;
  html.isNode = 1;

  return html;
}

CHtml script() {
  CHtmlUnion u;
  u.kind = Script;

  CHtml html;
  html.kid_count = 0;
  html.node = u;
  html.isNode = 1;

  return html;
}

CHtml select() {
  CHtmlUnion u;
  u.kind = Select;

  CHtml html;
  html.kid_count = 0;
  html.node = u;
  html.isNode = 1;

  return html;
}

CHtml aside() {
  CHtmlUnion u;
  u.kind = Aside;

  CHtml html;
  html.kid_count = 0;
  html.node = u;
  html.isNode = 1;

  return html;
}

CHtml nav() {
  CHtmlUnion u;
  u.kind = Nav;

  CHtml html;
  html.kid_count = 0;
  html.node = u;
  html.isNode = 1;

  return html;
}

CHtml a() {
  CHtmlUnion u;
  u.kind = Div;

  CHtml html;
  html.kid_count = 0;
  html.node = u;
  html.isNode = 1;

  return html;
}

CHtml text() {
  CHtmlUnion u;
  CHtmlText text;
  text.len = 0;
  u.text = text;

  CHtml html;
  html.kid_count = 0;
  html.node = u;
  html.isNode = 1;

  return html;
}

CHtml simple_test() {
  CHtml _text = text();
  _text.node.text.text = "Hello, World!";
  _text.node.text.len = sizeof(_text.node.text.text);

  CHtml _p = p();
  _p.kid_count = 1;
  _p.kids = &_text;

  CHtml _div = div();
  _div.kid_count = 1;
  _div.kids = &_p;

  return _div;
}

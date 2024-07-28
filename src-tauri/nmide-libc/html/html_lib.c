#include "html_lib.h"

CHtml create_chtml(CElement kind, CHtml *kids, int kid_count) {
  CHtml html;

  html.kind = kind;
  html.kids = kids;
  html.kid_count = kid_count;

  return html;
}

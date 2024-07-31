#include "css_lib.h"

int cap_int(int i, int cap) {
  if (i > cap) {
    return cap;
  } else if (i < 0) {
    return 0;
  } else {
    return i;
  }
}

CColor CColorNew(int r, int g, int b) {
  CColor color;

  color.R = cap_int(r, 255);
  color.G = cap_int(g, 255);
  color.B = cap_int(b, 255);

  return color;
}

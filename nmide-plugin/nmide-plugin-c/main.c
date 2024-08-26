#include "main.h"

static size_t count_digits(int num) {
  if (num == 0) {
    return 1;
  }
  size_t count = 0;
  if (num < 0) {
    num = -num;
    count++;
  }
  while (num >= 10) {
    num /= 10;
    count++;
  }
  count++;
  return count;
}

CMap cinit() {
  CMap *map_ptr = create_cmap();
  cmap_insert(map_ptr, new_val(Int, (void *)0), "c-counter");
  cmap_insert(map_ptr, new_val(Str, "c"), "nmide-type");
  CMap map = *map_ptr;
  return map;
}

CHtml cview(CMsg cmsg, CMap cmodel) {
  int count;
  if (strcmp("c-counter", cmsg.msg)) {
    MaybeVal *val = (MaybeVal *)malloc(sizeof(MaybeVal));
    val = cmap_lookup(&cmodel, "c-counter");
    if (val->just && val->val.type == Int) {
      val->val.val._int++;
    } else {
      CValUnion u = {._int = 1};
      val->val.val = u;
      val->val.type = Int;
    }
    count = val->val.val._int;
    cmap_remove(&cmodel, "c-counter");
    cmap_insert(&cmodel, &val->val, "c-counter");
  }
  char *txt = (char *)malloc(sizeof(char) * count_digits(count));
  sprintf(txt, "%d", count);
  CHtml *children = (CHtml *)malloc(sizeof(CHtml));
  CHtml text = {.isElement = false,
                .content = {
                    .text = txt,
                }};
  children[0] = text;
  CHtml btn = {
      .content = {.element =
                      {
                          .tag = Button,
                          .len = 1,
                          .children = children,
                      }},
      .isElement = true,
  };
  return btn;
}

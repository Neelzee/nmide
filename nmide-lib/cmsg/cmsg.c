#include "cmsg.h"

CMsg *new_cmsg(char *msg, CVal *val) {
  CMsg *m = (CMsg *)malloc(sizeof(CMsg));
  m->msg = msg;
  m->len = strlen(msg);
  m->opt = maybe(val);
  return m;
}

bool cmsg_kind_cmp(CMsg *a, CMsg *b) { return strcmp(a->msg, b->msg); }

void drop_cmsg(CMsg *msg) { free(msg); }

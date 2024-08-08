#ifndef CMSG

#define CMSG

#include "../map/cmap.h"

typedef struct CMsg {
  char *msg;
  size_t len;
  MaybeVal *opt;
} CMsg;

CMsg *new_cmsg(char *msg, CVal *val);

bool cmsg_kind_cmp(CMsg *a, CMsg *b);

void drop_cmsg(CMsg *msg);

#endif // !CMSG

#include "cmap.h"

CMap *create_cmap() {
  CMap *map = (CMap *)malloc(sizeof(CMap));

  map->val_len = 0;
  map->values = NULL;

  return map;
}

CKey **get_keys(CMap *self) {
  CKey **keys = (CKey **)malloc(self->val_len * sizeof(CKey *));

  for (int i = 0; i < self->val_len; i++) {
    keys[i] = self->values[i]->key;
  }

  return keys;
}

bool cmap_insert(CMap *self, CVal *val, CKey *key) {
  bool has_key = false;

  for (int i = 0; i < self->val_len; i++) {
    if (self->values[i]->key->key == key->key) {
      has_key = true;
      self->values[i]->val = val;
    }
  }

  if (!has_key) {
    CTuple *m_val = new_ctuple();
    m_val->key = key;
    m_val->val = val;
    self->val_len++;
    CTuple **new_values = (CTuple **)malloc(self->val_len * sizeof(CTuple *));
    for (int i = 0; i < self->val_len - 1; i++) {
      new_values[i] = self->values[i];
    }
    new_values[self->val_len - 1] = m_val;
    self->values = new_values;
  }

  return has_key;
}

MaybeVal *cmap_lookup(CMap *self, CKey *key) {
  MaybeVal *m = (MaybeVal *)malloc(sizeof(MaybeVal));

  m->just = false;
  m->val = (CVal *)malloc(sizeof(CVal));

  for (int i = 0; i < self->val_len; i++) {
    if (self->values[i]->key->key == key->key) {
      m->val = self->values[i]->val;
      m->just = true;
      return m;
    }
  }

  return m;
}

MaybeVal *cmap_remove(CMap *self, CKey *key) {
  MaybeVal *m = (MaybeVal *)malloc(sizeof(MaybeVal));
  m->just = false;
  m->val = (CVal *)malloc(sizeof(CVal));

  if (self->val_len == 0) {
    return m;
  }

  for (int i = 0; i < self->val_len; i++) {
    if (self->values[i]->key->key == key->key) {
      m->val = self->values[i]->val;
      m->just = true;
      self->values[i] = NULL;
      break;
    }
  }

  if (m->just) {
    self->val_len--;
  }

  return m;
}

CVal *new_val(CValType type, void *val) {
  CVal *res = (CVal *)malloc(sizeof(CVal));

  res->val = (CValUnion *)malloc(sizeof(CValUnion));

  res->type = type;

  switch (type) {
  case Str:
    res->val->str = (char *)val;
    break;
  case Int:
    res->val->_int = *(int *)val;
    break;
  case Arr:
    res->val->arr = (CArr *)val;
    break;
  case Obj:
    res->val->obj = (CMap *)val;
    break;
  }

  return res;
}

CVal *empty_val() { return new_val(Str, ""); }

MaybeVal *new_maybe() {
  MaybeVal *m = (MaybeVal *)malloc(sizeof(MaybeVal));

  m->just = false;
  m->val = (CVal *)malloc(sizeof(CVal));

  return m;
}

CKey *new_ckey() {
  CKey *k = (CKey *)malloc(sizeof(CKey));

  k->key = "";
  k->len = 0;

  return k;
}

void change_key(CKey *self, char *new_key) {
  self->key = new_key;
  self->len = strlen(new_key);
}

CTuple *new_ctuple() {
  CTuple *val = (CTuple *)malloc(sizeof(CTuple));

  val->key = new_ckey();
  val->val = empty_val();

  return val;
}

CArr *new_arr() {
  CArr *arr = (CArr *)malloc(sizeof(CArr));

  arr->elements = NULL;
  arr->len = 0;

  return arr;
}

void insert_arr(CArr *self, CVal *val) {
  CVal **new_arr = (CVal **)malloc(sizeof(CVal) * (self->len + 1));
  for (int i = 0; i < self->len; i++) {
    new_arr[i] = self->elements[i];
  }
  new_arr[self->len] = val;
  self->elements = new_arr;
}

void free_cval(CVal *val) {
  switch (val->type) {
  case Str:
    free(val->val);
    break;
  case Int:
    break;
  case Arr:
    free_arr(val->val->arr);
    break;
  case Obj:
    free_map(val->val->obj);
    break;
  }
  free(val->val);
  free(val);
}

void free_maybe(MaybeVal *val) {
  if (val->just) {
    free_cval(val->val);
  }
  free(val);
}

void free_key(CKey *key) {
  free(key->key);
  free(key);
}

void free_tuple(CTuple *tuple) {
  free_key(tuple->key);
  free_cval(tuple->val);
}

void free_map(CMap *map) { free(map); }

void free_arr(CArr *arr) { free(arr); }

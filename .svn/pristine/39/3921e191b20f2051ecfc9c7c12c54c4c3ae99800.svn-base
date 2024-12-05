#include "cmap.h"

// CMap ========================================================================

CMap *create_cmap() {
  CMap *map = (CMap *)malloc(sizeof(CMap));
  map->values = (CKeyPair *)malloc(sizeof(CKeyPair));
  map->len = 0;
  return map;
}

bool cmap_insert(CMap *self, CVal *val, char *key) {
  CKeyPair pair = self->values[0];
  bool has_key = false;
  for (int i = 0; i < self->len; i++) {
    pair = self->values[i];
    if (strcmp(pair.key, key)) {
      CVal old_val = pair.val;
      pair.val = *val;
      free_cval(val);
    }
  }
  if (has_key) {
    return true;
  }
  CKeyPair *new_values = (CKeyPair *)malloc(sizeof(CKeyPair) * (self->len + 1));
  for (int j = 0; j < self->len; j++) {
    new_values[j] = self->values[j];
  }
  CKeyPair *new_pair = key_pair(key, val);
  new_values[self->len - 1] = *new_pair;
  free(new_pair);
  self->values = new_values;
  return false;
}

MaybeVal *cmap_lookup(CMap *self, char *key) {
  CKeyPair pair = self->values[0];
  for (int i = 0; i < self->len; i++) {
    if (strcmp(pair.key, key)) {
      CVal *v = (CVal *)malloc(sizeof(CVal));
      v = &pair.val;
      return maybe(v);
    }
    i++;
    pair = self->values[i];
  }
  return maybe(NULL);
}

MaybeVal *cmap_remove(CMap *self, char *key) {
  MaybeVal *m = maybe(NULL);
  size_t index = 0;
  CKeyPair pair = self->values[0];
  for (int i = 0; i < self->len; i++) {
    if (strcmp(pair.key, key)) {
      CVal val = pair.val;
      m->just = true;
      m->val = val;
      index = i;
    }
    pair = self->values[i];
  }
  if (m->just) {
    CKeyPair *new_values = (CKeyPair *)malloc(sizeof(CKeyPair) * self->len);
    for (int j = 0; j < self->len; j++) {
      if (j == index) {
        continue;
      }
      new_values[j] = self->values[j];
    }
    self->values = new_values;
  }
  return m;
}

// =============================================================================

// CArr ========================================================================

void insert_arr(CArr *self, CVal *val) {
  CVal *new_elems = (CVal *)malloc(sizeof(CVal) * (self->len + 1));
  for (int i = 0; i < self->len; i++) {
    new_elems[i] = self->elements[i];
  }
  new_elems[self->len - 1] = *val;
  free(val);
  free(self->elements);
  self->elements = new_elems;
}

CArr *new_arr(CVal **elements) {
  CArr *arr = (CArr *)malloc(sizeof(CArr));
  if (elements == NULL) {
    elements = (CVal **)malloc(sizeof(CVal *));
    elements[0] = NULL;
  }
  arr->elements = *elements;
  free(elements);
  return arr;
}

CVal *carr_get(CArr *self, size_t i) {
  CVal *v = (CVal *)malloc(sizeof(CVal));

  if (self->len >= i) {
    v = &self->elements[i];
    return v;
  } else {
    free(v);
    return NULL;
  }
}

CVal *carr_remove(CArr *self, size_t i) {
  size_t size = self->len;
  CVal *val = (CVal *)malloc(sizeof(CVal));
  if (size >= i) {
    return val;
  }
  CVal *new_elems = (CVal *)malloc(sizeof(CVal) * size);
  for (int j = 0; j < size - 1; j++) {
    if (j == i) {
      val = &self->elements[i];
      continue;
    }
    new_elems[j] = self->elements[j];
  }
  self->elements = new_elems;
  return val;
}

// =============================================================================

MaybeVal *maybe(CVal *val) {
  MaybeVal *m = (MaybeVal *)malloc(sizeof(MaybeVal));
  m->val = *val;
  free(val);
  m->just = val != NULL;
  return m;
}

CVal *new_val(CValType type, void *val) {
  CVal *v = (CVal *)malloc(sizeof(CVal));
  v->type = type;
  CValUnion *u = (CValUnion *)malloc(sizeof(CValUnion));
  v->val = *u;
  free(u);
  switch (type) {
  case Str:
    u->str = (char *)val;
    return v;
  case Int:
    u->_int = *(int *)val;
    return v;
  case Arr:
    u->arr = *(CArr *)val;
    free(val);
    return v;
  case Obj:
    u->obj = *(CMap *)val;
    free(val);
    return v;
  }
}

CKeyPair *key_pair(char *_key, CVal *val) {
  CKeyPair *pair = (CKeyPair *)malloc(sizeof(CKeyPair));
  pair->key = _key;
  pair->val = *val;
  free(val);
  return pair;
}

void free_cval(CVal *val) {
  if (val != NULL) {
    free(val);
    val = NULL;
  }
}

void free_maybe(MaybeVal *val) {
  if (val != NULL) {
    free(val);
    val = NULL;
  }
}

void free_keypair(CKeyPair *pair) {
  if (pair != NULL) {
    free(pair);
    pair = NULL;
  }
}

void free_arr(CArr *arr) {
  if (arr != NULL) {
    if (arr->elements != NULL) {
      free(arr->elements);
      arr->elements = NULL;
    }
    free(arr);
    arr = NULL;
  }
}

void free_map(CMap *map) {
  if (map != NULL) {
    if (map->values != NULL) {
      free(map->values);
      map->values = NULL;
    }
    free(map);
    map = NULL;
  }
}

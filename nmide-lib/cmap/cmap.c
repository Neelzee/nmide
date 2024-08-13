#include "cmap.h"

// CMap ========================================================================

size_t map_size(CMap *self) {
  if (self == NULL) {
    return 0;
  }
  size_t i = 0;
  CKeyPair *pair = self->values[i];
  while (pair != NULL) {
    i++;
    pair = self->values[i];
  }
  return i;
}

CMap *create_cmap() {
  CMap *map = (CMap *)malloc(sizeof(CMap));
  CKeyPair **values = (CKeyPair **)malloc(sizeof(CKeyPair *));
  values[0] = NULL;
  map->values = values;
  return map;
}

bool cmap_insert(CMap *self, CVal *val, char *key) {
  size_t i = 0;
  CKeyPair *pair = self->values[i];
  bool has_key = false;
  while (pair != NULL) {
    if (strcmp(pair->key->key, key)) {
      CVal *old_val = pair->val;
      pair->val = val;
      free_cval(old_val);
    }
    i++;
    pair = self->values[i];
  }
  if (has_key) {
    return true;
  }
  CKeyPair **new_values = (CKeyPair **)malloc(sizeof(CKeyPair *) * (i + 2));
  for (int j = 0; j < i + 1; j++) {
    new_values[j] = self->values[j];
  }
  new_values[i - 2] = key_pair(key, val);
  new_values[i - 1] = NULL;
  self->values = new_values;
  return false;
}

MaybeVal *cmap_lookup(CMap *self, char *key) {
  size_t i = 0;
  CKeyPair *pair = self->values[i];
  while (pair != NULL) {
    if (strcmp(pair->key->key, key)) {
      return maybe(pair->val);
    }
    i++;
    pair = self->values[i];
  }
  return maybe(NULL);
}

MaybeVal *cmap_remove(CMap *self, char *key) {
  MaybeVal *m = maybe(NULL);
  size_t i = 0;
  size_t index = 0;
  CKeyPair *pair = self->values[i];
  while (pair != NULL) {
    if (strcmp(pair->key->key, key)) {
      CVal *val = pair->val;
      m->just = true;
      m->val = val;
      index = i;
      free_key(pair->key);
      pair->key = NULL;
      free(pair);
      pair = NULL;
    }
    i++;
    pair = self->values[i];
  }
  if (m->just) {
    CKeyPair **new_values = (CKeyPair **)malloc(sizeof(CKeyPair *) * i);
    for (int j = 0; j < i; j++) {
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

size_t arr_size(CArr *self) {
  if (self == NULL) {
    return 0;
  }
  size_t i = 0;
  CVal *elem = self->elements[i];
  while (elem != NULL) {
    i++;
    elem = self->elements[i];
  }
  return i;
}

void insert_arr(CArr *self, CVal *val) {
  size_t og_size = arr_size(self);
  CVal **new_elems = (CVal **)malloc(sizeof(new_elems) * (og_size + 2));
  for (int i = 0; i < og_size; i++) {
    new_elems[i] = self->elements[i];
  }
  new_elems[og_size - 2] = val;
  new_elems[og_size - 1] = NULL;
  free(self->elements);
  self->elements = new_elems;
}

CArr *new_arr(CVal **elements) {
  CArr *arr = (CArr *)malloc(sizeof(CArr));
  if (elements == NULL) {
    elements = (CVal **)malloc(sizeof(CVal *));
    elements[0] = NULL;
  }
  arr->elements = elements;
  return arr;
}

CVal *carr_get(CArr *self, size_t i) {
  return arr_size(self) >= i ? NULL : self->elements[i];
}

CVal *carr_remove(CArr *self, size_t i) {
  size_t size = arr_size(self);
  CVal *val = NULL;
  if (size >= i) {
    return val;
  }
  CVal **new_elems = (CVal **)malloc(sizeof(new_elems) * size);
  for (int j = 0; j < size - 1; j++) {
    if (j == i) {
      val = self->elements[i];
      continue;
    }
    new_elems[j] = self->elements[j];
  }
  new_elems[size] = NULL - 1;
  self->elements = new_elems;
  return val;
}

// =============================================================================

MaybeVal *maybe(CVal *val) {
  MaybeVal *m = (MaybeVal *)malloc(sizeof(MaybeVal));
  m->val = val;
  m->just = val != NULL;
  return m;
}

CVal *new_val(CValType type, void *val) {
  CVal *v = (CVal *)malloc(sizeof(CVal));
  v->type = type;
  CValUnion *u = (CValUnion *)malloc(sizeof(CValUnion));
  v->val = u;
  switch (type) {
  case Str:
    u->str = (char *)val;
    return v;
  case Int:
    u->_int = *(int *)val;
    return v;
  case Arr:
    u->arr = (CArr *)val;
    return v;
  case Obj:
    u->obj = (CMap *)val;
    return v;
  }
}

CKey *key(char *key) {
  CKey *k = (CKey *)malloc(sizeof(CKey));
  k->key = key;
  k->len = strlen(key);
  return k;
}

CKeyPair *key_pair(char *_key, CVal *val) {
  CKeyPair *pair = (CKeyPair *)malloc(sizeof(CKeyPair));
  pair->key = key(_key);
  pair->val = val;
  return pair;
}

void free_cval(CVal *val) {
  if (val != NULL) {
    if (val->val != NULL) {
      switch (val->type) {
      case Str:
        free((void *)val->val->str);
        val->val->str = NULL;
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
      val->val = NULL;
    }
    free(val);
    val = NULL;
  }
}

void free_maybe(MaybeVal *val) {
  if (val != NULL) {
    if (val->val != NULL) {
      free_cval(val->val);
      val->val = NULL;
    }
    free(val);
    val = NULL;
  }
}

void free_key(CKey *key) {
  if (key != NULL) {
    free(key);
    key = NULL;
  }
}

void free_keypair(CKeyPair *pair) {
  if (pair != NULL) {
    free_key(pair->key);
    pair->key = NULL;
    free_cval(pair->val);
    pair->val = NULL;
    free(pair);
    pair = NULL;
  }
}

void free_arr(CArr *arr) {
  if (arr != NULL) {
    if (arr->elements != NULL) {
      int i = 0;
      while (arr->elements[i] != NULL) {
        free_cval(arr->elements[0]);
        i++;
      }
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
      int i = 0;
      while (map->values[i] != NULL) {
        free_keypair(map->values[0]);
        i++;
      }
      free(map->values);
      map->values = NULL;
    }
    free(map);
    map = NULL;
  }
}

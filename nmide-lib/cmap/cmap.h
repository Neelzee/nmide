#ifndef CMAP

#define CMAP

#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct CVal CVal;
typedef struct CArr CArr;
typedef struct CMap CMap;

typedef union CValUnion {
  char *str;
  int _int;
  CArr *arr;
  CMap *obj;
} CValUnion;

struct CArr {
  CVal **elements;
};

typedef enum CValType {
  Str,
  Int,
  Arr,
  Obj,
} CValType;

struct CVal {
  CValType type;
  CValUnion *val;
};

typedef struct MaybeVal {
  bool just;
  CVal *val;
} MaybeVal;

typedef struct CKey {
  char *key;
  size_t len;
} CKey;

typedef struct CKeyPair {
  CKey *key;
  CVal *val;
} CKeyPair;

struct CMap {
  CKeyPair **values;
};

// TODO: Add doc-strings
size_t map_size(CMap *self);
size_t arr_size(CArr *self);
void insert_arr(CArr *self, CVal *val);

// TODO: Add doc-strings
// TODO: Implement
void free_cval(CVal *val);
void free_maybe(MaybeVal *val);
void free_key(CKey *key);
void free_keypair(CKeyPair *pair);
void free_map(CMap *map);
void free_arr(CArr *arr);

MaybeVal *maybe(CVal *val);

CVal *new_val(CValType type, void *val);

CArr *new_arr(CVal **elements);

CKey *key(char *key);

CKeyPair *key_pair(char *key, CVal *val);

CVal *carr_get(CArr *self, size_t i);
CVal *carr_remove(CArr *self, size_t i);

/**
 * Creates an empty cmap
 */
CMap *create_cmap();

/**
 * Inserts the given element to the map
 * Returning true if the key already existed
 */
bool cmap_insert(CMap *self, CVal *val, char *key);

/**
 * Returns the given element, if it exists
 */
MaybeVal *cmap_lookup(CMap *self, char *key);

/**
 * Removes the given element
 * Returns the element, if it exists
 */
MaybeVal *cmap_remove(CMap *self, char *key);

#endif // !CMAP

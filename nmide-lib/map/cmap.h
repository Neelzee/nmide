#ifndef CMAP

#define CMAP

#include <stdbool.h>
#include <stddef.h>
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
  size_t len;
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

typedef struct CTuple {
  CKey *key;
  CVal *val;
} CTuple;

struct CMap {
  CTuple **values;
  size_t val_len;
};

// TODO: Add doc-strings
CVal *new_val(CValType type, void *val);
CVal *empty_val();
MaybeVal *new_maybe();
CKey *new_ckey();
void change_key(CKey *self, char *new_key);
CTuple *new_ctuple();
CArr *new_arr();

void insert_arr(CArr *self, CVal *val);

// TODO: Add doc-strings
// TODO: Implement
void free_cval(CVal *val);
void free_maybe(MaybeVal *val);
void free_key(CKey *key);
void free_tuple(CTuple *tuple);
void free_map(CMap *map);
void free_arr(CArr *arr);

/**
 * Creates an empty cmap
 */
CMap *create_cmap();

/**
 * Gets all keys in the map
 */
CKey **get_keys(CMap *self);

/**
 * Inserts the given element to the map
 * Returning true if the key already existed
 */
bool cmap_insert(CMap *self, CVal *val, CKey *key);

/**
 * Returns the given element, if it exists
 */
MaybeVal *cmap_lookup(CMap *self, CKey *key);

/**
 * Removes the given element
 * Returns the element, if it exists
 */
MaybeVal *cmap_remove(CMap *self, CKey *key);

#endif // !CMAP

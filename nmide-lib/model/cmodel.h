#ifndef CMODEL

#define CMODEL

#include "../map/cmap.h"
#include <stdbool.h>
#include <stdlib.h>

typedef struct CModel {
  CMap *map;
} CModel;

/**
 * Creates std CModel used in Nmide
 */
CModel *init();

/**
 * Inserts the given element to the map
 * Returning true if the key already existed
 */
bool cmodel_insert(CModel *self, CVal *val, CKey *key);

/**
 * Returns the given element, if it exists
 */
MaybeVal *cmodel_lookup(CModel *self, CKey *key);

/**
 * Removes the given element
 * Returns the element, if it exists
 */
MaybeVal *cmodel_remove(CModel *self, CKey *key);

#endif // !CMODEL

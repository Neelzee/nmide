#ifndef CMODEL

#define CMODEL

#include "../cmap/cmap.h"
#include <stdbool.h>
#include <stdlib.h>

typedef struct CModel {
  CMap map;
} CModel;

/**
 * Creates std CModel used in Nmide
 */
CModel *cmodel_init();

void drop(CModel *model);

/**
 * Inserts the given element to the map
 * Returning true if the key already existed
 */
bool cmodel_insert(CModel *self, CVal *val, char *key);

/**
 * Returns the given element, if it exists
 */
MaybeVal *cmodel_lookup(CModel *self, char *key);

/**
 * Removes the given element
 * Returns the element, if it exists
 */
MaybeVal *cmodel_remove(CModel *self, char *key);

#endif // !CMODEL

#include "cmodel.h"

CModel *cmodel_init() {
  CMap *map = create_cmap();
  CModel *model = (CModel *)malloc(sizeof(CModel));
  model->map = map;
  cmap_insert(map, new_val(Arr, new_arr(NULL)), "location");
  return model;
}

void drop(CModel *model) {
  if (model == NULL) {
    return;
  }
  free_map(model->map);
  free(model);
  model = NULL;
}

bool cmodel_insert(CModel *self, CVal *val, char *key) {
  return cmap_insert(self->map, val, key);
}

MaybeVal *cmodel_lookup(CModel *self, char *key) {
  return cmap_lookup(self->map, key);
}

MaybeVal *cmodel_remove(CModel *self, char *key) {
  return cmap_remove(self->map, key);
}

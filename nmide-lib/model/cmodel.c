#include "cmodel.h"

CModel *init() {
  CMap *map = create_cmap();
  CKey *location_key = new_ckey();
  change_key(location_key, "location");
  CVal *location_arr = new_val(Arr, new_arr());
  CVal *main_location = new_val(Str, "main");
  insert_arr(location_arr->val->arr, main_location);
  CModel *model = (CModel *)malloc(sizeof(CModel));
  bool _ = cmodel_insert(model, location_arr, location_key);
  return model;
}

bool cmodel_insert(CModel *self, CVal *val, CKey *key) {
  return cmap_insert(self->map, val, key);
}

MaybeVal *cmodel_lookup(CModel *self, CKey *key) {
  return cmap_lookup(self->map, key);
}

MaybeVal *cmodel_remove(CModel *self, CKey *key) {
  return cmap_remove(self->map, key);
}

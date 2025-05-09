export const handlerRegistration = async (
  module: string,
  event_name?: string,
  module_name?: string
) => {
  if (event_name !== undefined) {
    let list = window.__nmideConfig__.handlerRegister.event.get(event_name)
    list = list === undefined ? [] : list;
    list.push(module);
    window.__nmideConfig__.handlerRegister.event.set(event_name, list);
  }
  if (module_name !== undefined) {
    let list = window.__nmideConfig__.handlerRegister.module.get(module_name)
    list = list === undefined ? [] : list;
    list.push(module);
    window.__nmideConfig__.handlerRegister.module.set(module_name, list);
  }
}
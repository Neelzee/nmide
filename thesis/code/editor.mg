concept Editor = {
  type Action;
  function addText(): Action;
  function removeText(): Action;
  function redoAction(a: Action): Action;
  function combineAction(first: Action, second: Action): Action;
};

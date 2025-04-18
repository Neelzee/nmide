concept Editor = {
  type Action;
  function noop(): Action;
  function addText(): Action;
  function removeText(): Action;
  function redoAction(a: Action): Action;
  function combineAction(first: Action, second: Action): Action;
  axiom noop(a: Action) {
    assert combineAction(noop(), a) == a;
  };
};

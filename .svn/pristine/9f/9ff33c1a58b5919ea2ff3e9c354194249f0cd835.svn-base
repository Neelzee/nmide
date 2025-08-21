concept Action = {
  type Action;
  function noop(): Action;
  function addText(): Action;
  function removeText(): Action;
  function redoAction(a: Action): Action;
  function combineAction(first: Action, second: Action): Action;
  axiom noop(a: Action) {
    assert combineAction(noop(), a) == a;
  };
  axiom combineAction(a: Action, b: Action, c: Action) {
      assert
        combineAction(a, combineAction(b, c))
          ==
        combineAction(combineAction(a, b), c);
  };
};

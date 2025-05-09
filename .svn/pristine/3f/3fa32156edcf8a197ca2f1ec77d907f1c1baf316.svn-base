concept NonEmptyList = {
  use List;
  type NonEmptyList;
  function build(t: T): NonEmptyList;
  function cons(xs: NonEmptyList, x: T): NonEmptyList;
  function head(xs: NonEmptyList): T;
  function concat(xs: NonEmptyList, ys: NonEmptyList): NonEmptyList;
  function concat(xs: List, ys: NonEmptyList): NonEmptyList;
  function concat(xs: NonEmptyList, ys: List): NonEmptyList;
  axiom notEmpty(xs: NonEmptyList) {
      assert xs != nil();
  }
};

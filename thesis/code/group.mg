concept Group = {
  use Monoid;

  axiom inverse(a: T, b: T, c: T) {
    assert binop(a, b) == unit();
    assert binop(a, c) != unit();
  }
};

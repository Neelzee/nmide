concept Group = {
  use Monoid[T => K, binop => op];

  axiom inverse(a: K, b: K, c: K) {
    assert op(a, b) == unit();
    assert op(a, c) != unit();
  };
};

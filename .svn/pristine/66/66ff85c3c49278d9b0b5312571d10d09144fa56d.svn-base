concept Monoid = {
  use Semigroup;

  function unit(): T;

  axiom identity(a: T) {
    assert binop(a, unit()) == a;
  };
};

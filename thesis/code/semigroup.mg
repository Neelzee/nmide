concept Semigroup = {
  use Binop;

  axiom associative(a: T, b: T, c: T) {
    assert
      binop(a, binop(b, c))
        ==
      binop(binop(a, b), c);
  }
};

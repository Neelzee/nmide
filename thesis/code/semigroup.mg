concept Semigroup = {
  use Binop;
  // a + (b + c) == (a + b) + c
  axiom associative(a: T, b: T, c: T) {
    assert
      binop(a, binop(b, c))
        ==
      binop(binop(a, b), c);
  }
};

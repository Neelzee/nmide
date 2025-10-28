concept Semigroup = {
  use Magma[
    T => K,
    op => plus
  ];

  axiom assoc(
    a: K,
    b: K,
    c: K
  ) {
    assert
      plus(a, plus(b, c))
        ==
      plus(plus(a, b), c);
  };
};

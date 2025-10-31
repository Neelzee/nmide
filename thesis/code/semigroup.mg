concept Semigroup = {
  use Magma[
    T => Nat,
    op => plus
  ];

  axiom assoc(
    a: Nat,
    b: Nat,
    c: Nat
  ) {
    assert
      plus(plus(a, b), c)
        ==
      plus(a, plus(b, c));
  };
};

concept NaturalNumbers = {
  type Number;
  function Zero(): Number;
  function Succ(number: Number): Number;
  function _+_(a: Number, b: Number): Number;
  axiom unit(a: Number) {
    assert Zero() + Zero() == Zero();
    assert Zero() + a == a;
    assert a + Zero() == a;
  };
};

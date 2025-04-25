concept ListConat = {
  use List;
  use Monoid[
    T => List,
    unit => emptyList,
    binop => _++_
  ];
}

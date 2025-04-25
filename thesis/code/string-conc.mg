concept StringConat = {
  use String;
  use Monoid[
    T => String,
    unit => emptyString,
    binop => _+_
  ];
}

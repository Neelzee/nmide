concept Logging = {
  type K;
  type Log;

  function getValue(log: Log): K;

  use Semigroup[binop => combine, T => Log];
};

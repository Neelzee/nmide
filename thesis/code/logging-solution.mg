concept Logging = {
  type Log;

  use Semigroup[binop => combine, T => Log];
};

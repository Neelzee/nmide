struct CxxNaturalNumbers_impl {
  typedef int N;

  N zero() {
    return 0;
  }

  N succ(const& N number) {
    return number + 1;
  }

  N plus(const& N a, const &N b) {
    return a + b;
  }
};

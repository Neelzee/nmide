public static void main(String[] args) {
  List<Int> xs = new ArrayList() { 1, 2 };
  List<Int> ys = new ArrayList() { 3, 4 };
  List<Int> zs = new ArrayList() { 5, 6 };
  assert (xs.addAll(ys.addAll(zs)))
          .equals(xs.addAll(ys).addAll(zs));
}

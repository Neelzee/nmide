trait Monoid<A>: Semigroup<A> {
    fn identity() -> A;
}

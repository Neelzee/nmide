pub mod html;

pub mod css;

pub mod attr;

pub(crate) mod utils {
    pub(crate) fn lookup<K, V>(xs: &[(K, V)], x: K) -> Option<&V>
    where
        K: Eq,
    {
        match xs {
            [] => None,
            [(y, v), ..] if y == &x => Some(v),
            [_, ys @ ..] => lookup(ys, x),
        }
    }

    pub(crate) fn take_while<K, P>(xs: &[K], p: P) -> &[K]
    where
        P: Fn(&K) -> bool,
    {
        match xs {
            [x, ys @ ..] if p(x) => &xs[..1 + take_while(ys, p).len()],
            _ => &[],
        }
    }

    pub(crate) fn drop_while<K, P>(xs: &[K], p: P) -> &[K]
    where
        P: Fn(&K) -> bool,
    {
        match xs {
            [x, ys @ ..] if p(x) => drop_while(ys, p),
            _ => xs,
        }
    }

    pub(crate) fn fst<A, B>(pair: (A, B)) -> A {
        pair.0
    }

    pub(crate) fn snd<A, B>(pair: (A, B)) -> B {
        pair.1
    }

    /// Takes the first element (if any) where the predicate P, is true
    ///
    /// # Example
    /// ```rust
    /// let slice = [1, 1, 2, 3];
    /// let first = grab_first(&slice, |e| e % 2 == 0);
    /// assert_eq!(first, Some(2));
    ///
    /// let slice_2 = [1, 1, 1, 3];
    /// let none = grab_first(&slice, |e| e % 2 == 0);
    /// assert_eq!(none, None);
    /// ```
    pub(crate) fn grab_first<K, P>(xs: &[K], p: P) -> Option<&K>
    where
        P: Fn(&K) -> bool,
    {
        match xs {
            [] => None,
            [x, ..] if p(x) => Some(x),
            [_, ys @ ..] => grab_first(ys, p),
        }
    }
}

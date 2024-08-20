pub mod html;

pub mod css;

pub mod attr;

pub mod msg;

pub mod map;

pub mod interface;

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

    pub(crate) fn remove<K, V>(xs: &[(K, V)], x: K) -> Vec<(K, V)>
    where
        K: Eq + Clone,
        V: Clone,
    {
        match xs {
            [] => Vec::new(),
            [(y, _), ys @ ..] if y == &x => ys.to_vec(),
            [y, ys @ ..] => {
                let mut vec = remove(ys, x).to_vec();
                vec.push(y.clone());
                vec
            }
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

    pub(crate) fn drop_first<K, P>(xs: &[K], p: P) -> Vec<K>
    where
        K: Clone,
        P: Fn(&K) -> bool,
    {
        match xs {
            [] => Vec::new(),
            [x, ys @ ..] if p(x) => ys.to_vec(),
            [y, ys @ ..] => {
                let mut vec = drop_first(ys, p);
                vec.push(y.clone());
                vec
            }
        }
    }
}

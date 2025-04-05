use std::marker::PhantomData;

pub(crate) trait FieldAccessor<T> {
    type Value;
    fn get(instance: &T) -> &Self::Value;
    fn set(instance: T, new_value: Self::Value) -> T;
}

pub(crate) trait TreeMovement<T> {
    fn next(instance: &T) -> Vec<T>;
    fn set(instance: T, children: Vec<T>) -> T;
}

pub trait Instruction<T>
where
    T: TreeMovement<T>,
{
    fn apply(self, value: T) -> T;
}

#[allow(clippy::type_complexity)]
pub enum Ins<T, F, P>
where
    T: TreeMovement<T>,
    F: FieldAccessor<T>,
    P: Fn(&T) -> bool + Copy,
    F::Value: Clone,
{
    AddField {
        _field_type: PhantomData<F>,
        value: F::Value,
        pred: Box<P>,
    },
    SetField {
        _field_type: PhantomData<F>,
        new_value: F::Value,
        pred: Box<P>,
    },
    AppField {
        _field_type: PhantomData<F>,
        value: F::Value,
        pred: Box<P>,
        combine: Box<dyn Fn(F::Value, F::Value) -> F::Value>,
    },
    AddNode {
        node: T,
    },
    RemNode {
        pred: Box<P>,
    },
    Then {
        fst: Box<Ins<T, F, P>>,
        snd: Box<Ins<T, F, P>>,
    },
}

impl<T, F, P> Instruction<T> for Ins<T, F, P>
where
    F: FieldAccessor<T>,
    T: TreeMovement<T> + Clone,
    P: Fn(&T) -> bool + Copy,
    F::Value: Clone,
{
    fn apply(self, node: T) -> T {
        match self {
            Ins::AddField { value, pred, .. } => {
                if pred(&node) {
                    return F::set(node, value);
                }
                match bfs(&node, *pred) {
                    Some(new_node) => F::set(new_node, value),
                    None => node,
                }
            }
            Ins::SetField {
                new_value, pred, ..
            } => {
                if pred(&node) {
                    return F::set(node, new_value);
                }
                match bfs(&node, *pred) {
                    Some(new_node) => F::set(new_node, new_value),
                    None => node,
                }
            }
            Ins::AppField {
                value,
                pred,
                combine,
                ..
            } => {
                if pred(&node) {
                    let new_value = (*combine)(value, F::get(&node).clone());
                    return F::set(node, new_value);
                }
                match bfs(&node, *pred) {
                    Some(new_node) => {
                        let new_value = (*combine)(value, F::get(&new_node).clone());
                        F::set(new_node, new_value)
                    }
                    None => node,
                }
            }
            Ins::AddNode { node: new_node } => {
                let mut children = T::next(&node);
                children.push(new_node);
                T::set(node, children)
            }
            Ins::RemNode { pred } => map_bfs(&node, *pred, |n| {
                let children = T::next(&n).into_iter().filter(|k| !(*pred)(k)).collect();
                T::set(n, children)
            }),
            Ins::Then { fst, snd } => (*snd).apply((*fst).apply(node)),
        }
    }
}

impl<T, F, P> Ins<T, F, P>
where
    F: FieldAccessor<T>,
    T: TreeMovement<T> + Clone,
    P: Fn(&T) -> bool + Copy,
    F::Value: Clone,
{
    pub fn combine(self, other: Self) -> Self {
        Self::Then {
            fst: Box::new(self),
            snd: Box::new(other),
        }
    }
}

pub(crate) fn bfs<T, P>(node: &T, pred: P) -> Option<T>
where
    T: TreeMovement<T> + Clone,
    P: Fn(&T) -> bool + Copy,
{
    if pred(node) {
        return Some(node.clone());
    }

    fn inner<T, P>(kids: Vec<T>, pred: P) -> Option<T>
    where
        T: TreeMovement<T> + Clone,
        P: Fn(&T) -> bool + Copy,
    {
        match kids.iter().find(|n| pred(n)) {
            Some(n) => Some(n.clone()),
            None => kids.iter().find_map(|n| inner(T::next(n), pred)),
        }
    }

    inner(T::next(node), pred)
}

pub(crate) fn map_bfs<T, P, F>(node: &T, pred: P, f: F) -> T
where
    T: TreeMovement<T> + Clone,
    P: Fn(&T) -> bool + Copy,
    F: Fn(T) -> T + Copy,
{
    if pred(node) {
        f(node.clone())
    } else {
        T::set(
            node.clone(),
            T::next(node).iter().map(|n| map_bfs(n, pred, f)).collect(),
        )
    }
}

pub struct InsBuilder<T>
where
    T: TreeMovement<T>,
{
    instruction: Box<dyn Instruction<T>>,
}

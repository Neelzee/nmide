use std::marker::PhantomData;

pub struct Core;

pub struct CoreModification;

pub trait FieldAccessor<T> {
    type Value;
    fn get(instance: &T) -> &Self::Value;
    fn set(instance: T, new_value: Self::Value) -> T;
}

pub struct Ins<T, F>
where
    F: FieldAccessor<T>,
{
    pub(crate) field: PhantomData<F>,
    pub(crate) kind: Mods,
    pub(crate) p: Box<dyn Fn(T) -> bool>,
    #[allow(clippy::type_complexity)]
    pub(crate) f: Option<Box<dyn Fn(T, F::Value) -> T>>,
}

pub enum Mods {
    Remove,
    Append,
    Modify,
}

pub(crate) enum _Ins<T, F>
where
    F: FieldAccessor<T>,
{
    Rem {
        p: Box<dyn Fn(T) -> bool>,
    },
    Add {
        p: Box<dyn Fn(T) -> bool>,
        node: T,
    },
    Mod {
        field: PhantomData<F>,
        value: F::Value,
    },
}

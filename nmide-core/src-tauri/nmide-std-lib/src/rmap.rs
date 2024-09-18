use abi_stable::{
    std_types::{ROption, RString, RVec},
    StableAbi,
};
use std::mem::ManuallyDrop;

#[repr(C)]
#[derive(StableAbi)]
pub struct RValue {
    kind: RValKind,
    val: ManuallyDrop<RValueUnion>,
}

impl RValue {
    pub fn int(i: i32) -> Self {
        i.into()
    }

    pub fn float(f: f32) -> Self {
        f.into()
    }

    pub fn bool(b: bool) -> Self {
        b.into()
    }

    pub fn str<S: ToString>(s: S) -> Self {
        s.to_string().into()
    }

    pub fn list<T: Into<RValue>>(list: Vec<T>) -> Self {
        list.into()
    }
}

impl From<i32> for RValue {
    fn from(value: i32) -> Self {
        Self {
            kind: RValKind::Int,
            val: ManuallyDrop::new(RValueUnion::int(value)),
        }
    }
}

impl From<f32> for RValue {
    fn from(value: f32) -> Self {
        Self {
            kind: RValKind::Float,
            val: ManuallyDrop::new(RValueUnion::float(value)),
        }
    }
}

impl From<bool> for RValue {
    fn from(value: bool) -> Self {
        Self {
            kind: RValKind::Bool,
            val: ManuallyDrop::new(RValueUnion::bool(value)),
        }
    }
}

impl From<String> for RValue {
    fn from(value: String) -> Self {
        Self {
            kind: RValKind::Str,
            val: ManuallyDrop::new(RValueUnion::str(value)),
        }
    }
}

impl From<&str> for RValue {
    fn from(value: &str) -> Self {
        Self {
            kind: RValKind::Str,
            val: ManuallyDrop::new(RValueUnion::str(value)),
        }
    }
}

impl<T: Into<RValue>> From<Vec<T>> for RValue {
    fn from(value: Vec<T>) -> Self {
        Self {
            kind: RValKind::List,
            val: ManuallyDrop::new(RValueUnion::list(value)),
        }
    }
}

#[repr(C)]
#[derive(StableAbi)]
pub union RValueUnion {
    _int: i32,
    _float: f32,
    _bool: bool,
    _str: ManuallyDrop<RString>,
    _lst: ManuallyDrop<RVec<RValue>>,
    _obj: ManuallyDrop<RVec<RKeyPair>>,
}

impl RValueUnion {
    pub fn int(_int: i32) -> Self {
        Self { _int }
    }

    pub fn float(_float: f32) -> Self {
        Self { _float }
    }

    pub fn bool(_bool: bool) -> Self {
        Self { _bool }
    }

    pub fn str<S>(s: S) -> Self
    where
        S: ToString,
    {
        let mut rstr = RString::new();
        rstr.push_str(s.to_string().as_str());
        Self {
            _str: ManuallyDrop::new(rstr),
        }
    }

    pub fn list<T>(lst: Vec<T>) -> Self
    where
        T: Into<RValue>,
    {
        Self {
            _lst: ManuallyDrop::new(RVec::from_iter(lst.into_iter().map(|t| t.into()))),
        }
    }

    pub fn obj<S, T>(lst: Vec<(S, T)>) -> Self
    where
        S: ToString,
        T: Into<RValue>,
    {
        Self {
            _obj: ManuallyDrop::new(RVec::from_iter(lst.into_iter().map(|t| t.into()))),
        }
    }
}

#[repr(C)]
#[derive(StableAbi)]
pub struct RKeyPair {
    key: ManuallyDrop<RString>,
    val: ManuallyDrop<RValue>,
}

impl RKeyPair {
    pub fn cmp_key<S: ToString>(&self, key: &S) -> bool {
        *self.key == key.to_string()
    }
}

impl<S: ToString, T: Into<RValue>> From<(S, T)> for RKeyPair {
    fn from(value: (S, T)) -> Self {
        let (s, val) = value;
        let mut rstr = RString::new();
        rstr.push_str(s.to_string().as_str());
        Self {
            key: ManuallyDrop::new(rstr),
            val: ManuallyDrop::new(val.into()),
        }
    }
}

#[repr(u8)]
#[derive(StableAbi)]
pub enum RValKind {
    Int,
    Float,
    Bool,
    Str,
    List,
    Obj,
}

#[repr(C)]
#[derive(StableAbi)]
pub struct RMap {
    pairs: ManuallyDrop<RVec<RKeyPair>>,
}

impl RMap {
    pub fn new() -> RMap {
        RMap {
            pairs: ManuallyDrop::new(RVec::new()),
        }
    }

    pub fn insert<S, T>(self, key: S, val: T) -> Self
    where
        S: ToString,
        T: Into<RValue> + Clone,
    {
        Self {
            pairs: ManuallyDrop::new(
                ManuallyDrop::<RVec<RKeyPair>>::into_inner(self.pairs)
                    .into_iter()
                    .map(|mut pair| {
                        if pair.cmp_key(&key) {
                            let _ = ManuallyDrop::into_inner(pair.val);
                            pair.val = ManuallyDrop::new(val.clone().into());
                            return pair;
                        }
                        return pair;
                    })
                    .collect(),
            ),
        }
    }

    pub fn lookup<S: ToString>(&self, key: S) -> ROption<&RValue> {
        for p in self.pairs.iter() {
            if p.cmp_key(&key) {
                return ROption::RSome(&p.val);
            }
        }
        return ROption::RNone;
    }

    pub fn remove<S: ToString>(&mut self, key: S) -> ROption<ManuallyDrop<RValue>> {
        let mut index = 0;
        for p in self.pairs.iter() {
            if p.cmp_key(&key) {
                return ROption::RSome(self.pairs.swap_remove(index).val);
            }
            index += 1;
        }
        return ROption::RNone;
    }
}

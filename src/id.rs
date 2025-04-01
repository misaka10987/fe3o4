use super::{HasRegTab, Register};
use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Serialize,
};
use std::{
    borrow::Borrow,
    fmt::{Debug, Display},
    hash::Hash,
    marker::PhantomData,
    ops::Deref,
};

#[cfg(target_family = "wasm")]
use {tsify_next::Tsify, wasm_bindgen::prelude::wasm_bindgen};

/// This is the string identifier used to access reusable resource that is registered during the game.
///
/// A recommended naming style is to prefix the id with a namespace before a `:`, e.g. `modname:actual-id`.
#[derive(Clone, Copy, Serialize)]
#[serde(transparent)]
#[cfg_attr(target_family = "wasm", derive(Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
pub struct Id<T>(#[cfg_attr(target_family = "wasm", tsify(type = "string"))] IdInner<T>)
where
    T: 'static + Register;

impl<T> Id<T> {
    /// Interpret a string as an [`Id`].
    pub const fn new(id: &'static str) -> Self {
        Self(IdInner {
            id,
            _phantom: PhantomData,
        })
    }

    /// Resolve the local id part.
    pub fn get_id(&self) -> &'static str {
        self.0.id.split(':').last().unwrap()
    }

    /// Resolve the modname part.
    pub fn get_mod(&self) -> &'static str {
        self.0.id.split(':').next().unwrap()
    }
}

/// Seperate definition of this struct from [`Id`] is theoratically unnecessary.
/// Currently it is a walkaround to let `tsify-next` generate types with correct generic arguments.
#[derive(Serialize)]
#[serde(transparent)]
struct IdInner<T>
where
    T: Register,
{
    id: &'static str,
    // `serde_derive` intelligently skips its serialization.
    _phantom: PhantomData<T>,
}

impl<T: Register> Clone for IdInner<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

impl<T: Register> Copy for IdInner<T> {}

impl<T: Register> Deref for Id<T> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.id
    }
}

impl<T: Register> Borrow<str> for Id<T> {
    fn borrow(&self) -> &str {
        &self
    }
}

impl<T: Register> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.id)
    }
}

impl<T: Register> Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.id)
    }
}

impl<T: Register> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0.id == other.0.id
    }
}

impl<T: Register> Eq for Id<T> {}

impl<T: Register> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.id.partial_cmp(&other.0.id)
    }
}

impl<T: Register> Ord for Id<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.id.cmp(&other.0.id)
    }
}

impl<T: Register> Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.id.hash(state);
    }
}

impl<T: Register> From<&'static str> for Id<T> {
    fn from(value: &'static str) -> Self {
        Self::new(value)
    }
}

struct IdVisitor<T>(PhantomData<T>);

impl<'de, T: HasRegTab> Visitor<'de> for IdVisitor<T> {
    type Value = Id<T>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "an already registered id string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let tab = T::reg_tab();
        let res = tab.view(v, |k, _| *k);
        match res {
            Some(i) => Ok(Id::new(i)),
            None => Err(E::invalid_value(
                Unexpected::Str(v),
                &"an already registered id string",
            )),
        }
    }
}

impl<'de, T: HasRegTab> Deserialize<'de> for Id<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_str(IdVisitor::<T>(PhantomData))
    }
}

use std::{
    borrow::Borrow,
    fmt::{Debug, Display},
    hash::Hash,
    marker::PhantomData,
    ops::Deref,
};

use serde::{
    de::{Unexpected, Visitor},
    Deserialize, Serialize,
};

use super::{HasRegTab, Register};

/// Identifier for a registered item.
#[derive(PartialOrd, Serialize)]
#[serde(transparent)]
pub struct Id<T: Register> {
    id: &'static str,
    // `serde_derive` intelligently skips its serialization.
    _phantom: PhantomData<T>,
}

impl<T: Register> Id<T> {
    /// Interpret a string as an `Id`.
    pub const fn new(id: &'static str) -> Self {
        Self {
            id,
            _phantom: PhantomData,
        }
    }

    /// The identifier part.
    pub fn idname(&self) -> &'static str {
        self.id.split(':').last().unwrap()
    }

    /// The module name part.
    pub fn modname(&self) -> &'static str {
        self.id.split(':').next().unwrap()
    }
}

impl<T: Register> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

impl<T: Register> Copy for Id<T> {}

impl<T: Register> Deref for Id<T> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.id
    }
}

impl<T: Register> Borrow<str> for Id<T> {
    fn borrow(&self) -> &str {
        &self
    }
}

impl<T: Register> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.id)
    }
}

impl<T: Register> Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.id)
    }
}

impl<T: Register> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T: Register> Eq for Id<T> {}

impl<T: Register> Ord for Id<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl<T: Register> Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
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

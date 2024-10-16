pub mod id;
pub mod prelude;
pub mod tab;

use std::ops::Deref;

use dashmap::mapref::one::Ref;
pub use prelude::*;
use serde::{
    de::{DeserializeOwned, Error, Unexpected},
    Deserialize, Serialize,
};

/// A type being able to be registered.
pub trait Register: 'static + Sized {}
impl<T> Register for T where T: 'static + Sized {}

/// A smart pointer to an either registered resource or orphan.
/// 
/// This either refers to an item in registry table or holds an owned `T` instance.
pub enum Rp<T: Register> {
    /// The item is found in a registry table.
    Registered(Ref<'static, &'static str, T>),
    /// The item is orphan, or not in registry table.
    Orphan(Box<T>),
}

impl<T: HasRegTab> Deref for Rp<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            Rp::Registered(p) => p,
            Rp::Orphan(p) => p,
        }
    }
}

impl<T: Register> Clone for Rp<T>
where
    T: Clone + HasRegTab,
{
    fn clone(&self) -> Self {
        match self {
            Self::Registered(r) => Self::Registered(T::reg_tab().get(r.key()).unwrap()),
            Self::Orphan(v) => Self::Orphan(v.clone()),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
enum SerdeRp<T: HasRegTab> {
    #[serde(rename = "r")]
    Registered(Id<T>),
    #[serde(rename = "o")]
    Orphan(Box<T>),
}

impl<T> Serialize for Rp<T>
where
    T: Clone + Serialize + HasRegTab,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let serde = match self {
            Rp::Registered(id) => SerdeRp::Registered((*id.key()).into()),
            Rp::Orphan(v) => SerdeRp::Orphan(v.clone()),
        };
        serde.serialize(serializer)
    }
}

impl<'de, T> Deserialize<'de> for Rp<T>
where
    T: DeserializeOwned + HasRegTab,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let d = SerdeRp::deserialize(deserializer)?;
        match d {
            SerdeRp::Registered(id) => {
                let tab = T::reg_tab();
                let got = tab.get(&*id);
                if let Some(got) = got {
                    return Ok(Rp::Registered(got));
                }
                Err(D::Error::invalid_value(
                    Unexpected::StructVariant,
                    &"an orphan or registered id",
                ))
            }
            SerdeRp::Orphan(v) => Ok(Rp::Orphan(v)),
        }
    }
}

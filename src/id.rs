use arrayvec::{ArrayString, CapacityError};
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{
    any::type_name,
    fmt::{Debug, Display},
    hash::Hash,
    marker::PhantomData,
    ops::Deref,
    str::FromStr,
};

use crate::err::InvalidIdError;

/// A string identifier for a resource in the registry.
///
/// The type parameter `T` indicates type of the underlying resource.
///
/// # Specifications
///
/// An `Id` consists of two parts: a `module` part for namespacing uses and a `name` part for identification.
/// Each part is an ASCII string made up with uppercase or lowercase letters, digits, hyphen, period and underscore,
/// with a maximum length of 12. i.e. `^[a-zA-Z0-9._-]{1,12}$`.
///
/// The bahaviour is undefined unless conditions above are satisfied.
///
/// # String Representation
///
/// String representation for a certain `Id` would be the `module` part and `name` part concatenated by a slash (`/`).
/// e.g. `module/name`.
///
/// # Comparison
///
/// `Id` is guaranteed to have the same comparison behaviour as its corresponding string representation.
#[derive(SerializeDisplay, DeserializeFromStr)]
#[cfg_attr(target_family = "wasm", derive(tsify_next::Tsify))]
#[cfg_attr(target_family = "wasm", tsify(into_wasm_abi, from_wasm_abi))]
#[repr(transparent)]
pub struct Id<T>(#[cfg_attr(target_family = "wasm", tsify(type = "string"))] IdInner<T>);

impl<T> Id<T> {
    /// Create an `Id` with specified `module` and `name` part.
    pub const fn new(module: ArrayString<12>, name: ArrayString<12>) -> Self {
        Self(IdInner {
            module,
            name,
            _t: PhantomData,
        })
    }
}

/// Create an [`Id`] from string.
///
/// # Panics
///
/// This function panics if argument is not a valid [`Id`].
pub fn id<T>(id: &str) -> Id<T> {
    id.parse().unwrap()
}

impl<T> Deref for Id<T> {
    type Target = IdInner<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Clone for Id<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> Copy for Id<T> {}

impl<T> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> Eq for Id<T> {}

impl<T> PartialOrd for Id<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T> Ord for Id<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> Hash for Id<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.module.hash(state);
        self.name.hash(state);
    }
}

impl<T> Debug for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let t = type_name::<T>();
        write!(f, "Id::<{t}>(\"{self}\")")
    }
}

pub struct IdInner<T> {
    pub module: ArrayString<12>,
    pub name: ArrayString<12>,
    _t: PhantomData<T>,
}

impl<T> Clone for IdInner<T> {
    fn clone(&self) -> Self {
        Self {
            module: self.module.clone(),
            name: self.name.clone(),
            _t: PhantomData,
        }
    }
}

impl<T> Copy for IdInner<T> {}

impl<T> PartialEq for IdInner<T> {
    fn eq(&self, other: &Self) -> bool {
        self.module == other.module && self.name == other.name
    }
}

impl<T> Eq for IdInner<T> {}

impl<T> PartialOrd for IdInner<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for IdInner<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.module.cmp(&other.module) {
            std::cmp::Ordering::Equal => self.name.cmp(&other.name),
            x => x,
        }
    }
}

impl<T> Display for Id<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.module, self.name)
    }
}

impl<T> FromStr for Id<T> {
    type Err = InvalidIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let it = s.split('/');
        let parts = it.collect::<Vec<_>>();
        if parts.len() != 2 {
            return Err(InvalidIdError::InvalidParts);
        }
        let module = ArrayString::<12>::from(parts[0]).map_err(CapacityError::simplify)?;
        let name = ArrayString::<12>::from(parts[1]).map_err(CapacityError::simplify)?;
        Ok(Self::new(module, name))
    }
}

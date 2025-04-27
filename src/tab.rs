use std::ops::{Deref, DerefMut};

use dashmap::DashMap;

use crate::{Id, err::ResNotFoundError};

/// A registry for storing resources mapped by [`Id`].
///
/// The registry is generally a read-only hashmap with [`Id`]s as keys and underlying `T`s as values,
/// and can be accessed as a regular hashmap.
///
/// In order to create a `Registry`, use [`RegistryBuilder`].
pub struct Registry<T>(dashmap::ReadOnlyView<Id<T>, T>);

impl<T> Registry<T> {
    pub fn reg(&self, id: Id<T>) -> Result<&T, ResNotFoundError<T>> {
        self.get(&id).ok_or(id.into())
    }
}

impl<T> Deref for Registry<T> {
    type Target = dashmap::ReadOnlyView<Id<T>, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// A builder for [`Registry`].
///
/// This is generally a mutable hashmap.
/// Initialize a [`Registry`] by inserting into this builder and calling the [`Self::build`] method.
pub struct RegistryBuilder<T>(DashMap<Id<T>, T>);

impl<T> RegistryBuilder<T> {
    /// Create an empty builder.
    pub fn new() -> Self {
        Self(DashMap::new())
    }
    /// Create a [`Registry`].
    pub fn build(self) -> Registry<T> {
        Registry(self.0.into_read_only())
    }
}

impl<T> Deref for RegistryBuilder<T> {
    type Target = DashMap<Id<T>, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for RegistryBuilder<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

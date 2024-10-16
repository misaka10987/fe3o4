pub mod id;
pub mod prelude;
pub mod tab;

use std::ops::Deref;

use dashmap::mapref::one::Ref;
pub use prelude::*;

pub trait Register: 'static + Sized {}
impl<T> Register for T where T: 'static + Sized {}

/// A smart pointer to an either registered resource or orphan.
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

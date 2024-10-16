use std::ops::{Deref, DerefMut};

use dashmap::DashMap;

use super::Register;

pub struct RegTab<T: Register>(DashMap<&'static str, T>);

impl<T: Register> RegTab<T> {
    pub fn new() -> Self {
        Self(DashMap::new())
    }
}

/// A type having a registry table definition.
pub trait HasRegTab: Register {
    /// Registry table for this type.
    fn reg_tab() -> &'static RegTab<Self>;
}

impl<T: Register> Deref for RegTab<T> {
    type Target = DashMap<&'static str, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Register> DerefMut for RegTab<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[macro_export]
macro_rules! has_regtab {
    ($t:ty,$e:expr) => {
        impl $crate::HasRegTab for $t {
            fn reg_tab() -> &'static $crate::RegTab<Self> {
                &$e
            }
        }
    };
}

/// # Important
/// Add `static_init` to your dependencies to use this macro.
#[macro_export]
macro_rules! def_regtab {
    ($t:ty,$i:ident) => {
        #[::static_init::dynamic]
        static $i: $crate::RegTab<$t> = $crate::RegTab::new();
        $crate::has_regtab!($t, $i);
    };
}

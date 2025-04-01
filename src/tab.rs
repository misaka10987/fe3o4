use std::ops::{Deref, DerefMut};

use dashmap::DashMap;

use super::Register;

/// A registry table to store mappings from string IDs to entries.
pub struct RegTab<T: Register>(DashMap<&'static str, T>);

impl<T: Register> RegTab<T> {
    /// Create an empty registry table.
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

/// Implement `HasRegTab` for a certain type with provided registry table.
#[macro_export]
macro_rules! has_regtab {
    ($t:ty,$e:expr) => {
        impl $crate::HasRegTab for $t {
            #[inline]
            fn reg_tab() -> &'static $crate::RegTab<Self> {
                &$e
            }
        }
    };
}

/// Define a static variable as registry table for specified type,
/// automatically calling `has_regtab`.
///
/// `_REGTAB` is used as name of the variable unless an identifier is supplied via arguments,
/// which may lead to naming conflicts.
/// In such case, supply a name manually.
///
/// # Example
/// ```
/// use fe3o4::def_regtab;
///
/// struct Foo {};
/// struct Bar {};
///
/// def_regtab!(Foo);
/// def_regtab!(Bar, REG_BAR);
/// ```
///
/// # `static_init` Support
///
/// The default implementation uses [`std::sync::LazyLock`] for handling the static initialization of the registry table.
/// However, `static_init` crate can be optionally used as an alternative, with the following steps:
/// - Enable the `static-init` feature for this crate
/// - **(IMPORTANT)** Add `static_init` to YOUR dependencies (since re-exports of proc-macros are currently not supported)
#[macro_export]
macro_rules! def_regtab {
    ($t:ty) => {
        def_regtab!($t, _REGTAB);
    };
    ($t:ty, $i:ident) => {
        $crate::def_regtab_impl!($t, $i);
    };
}

#[macro_export]
#[cfg(feature = "static-init")]
macro_rules! def_regtab_impl {
    ($t:ty,$i:ident) => {
        #[::static_init::dynamic]
        static $i: $crate::RegTab<$t> = $crate::RegTab::new();
        $crate::has_regtab!($t, $i);
    };
}

#[macro_export]
#[cfg(not(feature = "static-init"))]
macro_rules! def_regtab_impl {
    ($t:ty,$i:ident) => {
        static $i: std::sync::LazyLock<$crate::RegTab<$t>> =
            std::sync::LazyLock::new(|| $crate::RegTab::new());
        $crate::has_regtab!($t, $i);
    };
}

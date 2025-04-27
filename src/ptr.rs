use serde::{Deserialize, Serialize};

use crate::Id;

#[derive(Clone, Serialize, Deserialize)]
pub enum RegPtr<T> {
    Registered(Id<T>),
    Orphan(Box<T>),
}

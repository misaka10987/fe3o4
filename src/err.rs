use thiserror::Error;

use crate::Id;

#[derive(Debug, Error)]
pub enum InvalidIdError {
    /// The [`Id`] is too long.
    #[error(transparent)]
    Length(#[from] arrayvec::CapacityError),
    /// The [`Id`] does not contain two valid parts.
    #[error("should contain one and only '/' as separator")]
    InvalidParts,
}

#[derive(Debug, Error)]
#[error("resource not found for {id}")]
pub struct ResNotFoundError<T> {
    #[from]
    id: Id<T>,
}

//! Contains [`AllSomeIter`], for coercing a slice of options to an [`Iterator`] of non-options.
use std::{iter::Flatten, slice::Iter};
use thiserror::Error;

/// Error that can occur when attempting to construct [AllSomeIter].
#[derive(Debug, Error)]
#[error("Option<T> collection contains one or more Nones")]
pub struct NotAllSomeError;

/// [`Iterator`] for coercing a slice of options to non-options.
///
/// Can only be constructed via [`TryFrom<&[Option<T>]>`](TryFrom).
/// This will error if any of the elements are `None`.
/// Otherwise, this will iterate over the non-optional `T`.
pub struct AllSomeIter<'a, T> {
    flattened: Flatten<Iter<'a, Option<T>>>,
}

impl<'a, T> Iterator for AllSomeIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.flattened.next()
    }
}

impl<'a, T> TryFrom<&'a [Option<T>]> for AllSomeIter<'a, T> {
    type Error = NotAllSomeError;

    fn try_from(value: &'a [Option<T>]) -> Result<Self, Self::Error> {
        if value.iter().all(|v| v.is_some()) {
            Ok(AllSomeIter {
                flattened: value.iter().flatten(),
            })
        } else {
            Err(NotAllSomeError)
        }
    }
}

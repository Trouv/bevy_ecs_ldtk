use std::{iter::Flatten, slice::Iter};
use thiserror::Error;

#[derive(Debug, Error)]
#[error("Option<T> iterator contains one or more Nones")]
pub struct NotAllSomeError;

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

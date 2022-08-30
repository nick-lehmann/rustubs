#![allow(dead_code)]
use core::fmt::Display;

#[repr(transparent)]
pub struct ExceptionID(u8);

impl ExceptionID {
    /// Check if the given id is a valid exception id.
    pub fn check<T>(id: T) -> bool
    where
        T: Into<u8>,
    {
        let x: u8 = id.into();
        (0u8..=31).contains(&x)
    }

    /// Construct an `ExceptionID` from an integer, including validation.
    pub fn from<T>(id: T) -> ExceptionID
    where
        T: Into<u8> + Display + Clone,
    {
        match ExceptionID::check(id.clone()) {
            true => ExceptionID(id.into()),
            false => panic!("{} is not a valid exception id", id),
        }
    }

    /// Blindly construct an `ExceptionId` from an integer and skip validation.
    pub unsafe fn from_unchecked<T>(id: T) -> ExceptionID
    where
        T: Into<u8>,
    {
        ExceptionID(id.into())
    }

    /// Convert exception to usize.
    ///
    /// Useful for using this wrapper as an index.
    pub fn as_usize(&self) -> usize {
        self.0 as usize
    }
}

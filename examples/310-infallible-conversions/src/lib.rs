#![allow(clippy::all)]
//! # Infallible Conversions with Into
//!
//! `Infallible` marks conversions that cannot fail.

use std::convert::TryFrom;

/// Non-zero wrapper with fallible construction
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NonZeroU32(u32);

#[derive(Debug, PartialEq)]
pub struct ZeroError;

impl TryFrom<u32> for NonZeroU32 {
    type Error = ZeroError;
    fn try_from(n: u32) -> Result<Self, ZeroError> {
        if n == 0 {
            Err(ZeroError)
        } else {
            Ok(NonZeroU32(n))
        }
    }
}

impl NonZeroU32 {
    pub fn get(&self) -> u32 {
        self.0
    }
}

/// Newtype with infallible From conversion
#[derive(Debug, PartialEq)]
pub struct Meters(pub f64);

impl From<f64> for Meters {
    fn from(v: f64) -> Self {
        Meters(v)
    }
}

/// Demonstrate infallible conversions
pub fn u8_to_u32(small: u8) -> u32 {
    u32::from(small) // always succeeds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nonzero_ok() {
        assert_eq!(NonZeroU32::try_from(5), Ok(NonZeroU32(5)));
    }

    #[test]
    fn test_nonzero_err() {
        assert_eq!(NonZeroU32::try_from(0), Err(ZeroError));
    }

    #[test]
    fn test_from_infallible() {
        assert_eq!(u8_to_u32(255), 255);
    }

    #[test]
    fn test_meters_from() {
        let m: Meters = Meters::from(5.0);
        assert_eq!(m, Meters(5.0));
    }

    #[test]
    fn test_into() {
        let m: Meters = 100.0f64.into();
        assert_eq!(m.0, 100.0);
    }
}

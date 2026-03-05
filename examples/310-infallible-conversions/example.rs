//! 310. Infallible conversions with Into
//!
//! `Infallible` marks conversions that cannot fail; `From<T>` implies `TryFrom<T, Error=Infallible>`.

use std::convert::{TryFrom, TryInto, Infallible};

/// Custom type that wraps a non-zero u32
#[derive(Debug, Clone, Copy, PartialEq)]
struct NonZeroU32(u32);

#[derive(Debug, PartialEq)]
struct ZeroError;

impl TryFrom<u32> for NonZeroU32 {
    type Error = ZeroError;
    fn try_from(n: u32) -> Result<Self, ZeroError> {
        if n == 0 { Err(ZeroError) } else { Ok(NonZeroU32(n)) }
    }
}

/// Infallible conversion: u8 always fits in u32
/// This is provided automatically via From<u8> for u32
#[allow(dead_code)]
fn demonstrate_from() {
    let small: u8 = 255;
    let big: u32 = u32::from(small); // infallible
    println!("u8 {} -> u32 {}", small, big);

    // TryFrom on u32->u8 is fallible
    let large: u32 = 300;
    let result = u8::try_from(large);
    println!("u32 {} -> u8: {:?}", large, result);
}

/// Newtype with infallible From conversion
#[derive(Debug, PartialEq)]
struct Meters(f64);

impl From<f64> for Meters {
    fn from(v: f64) -> Self { Meters(v) }
}

// From<f64> automatically gives us TryFrom<f64, Error=Infallible>
// We can verify:
fn use_try_from(val: f64) -> Result<Meters, Infallible> {
    Meters::try_from(val)
}

fn main() {
    // Standard library From impls
    let n: u32 = u32::from(42u8); // infallible
    println!("u8 -> u32: {}", n);

    let s: String = String::from("hello"); // infallible
    println!("&str -> String: {}", s);

    // TryFrom fallible
    let ok = NonZeroU32::try_from(5u32);
    let err = NonZeroU32::try_from(0u32);
    println!("TryFrom 5: {:?}", ok);
    println!("TryFrom 0: {:?}", err);

    // Into (reverse of From)
    let m: Meters = 100.0f64.into(); // via From<f64>
    println!("Meters: {:?}", m);

    // use_try_from via TryFrom auto-impl
    let result = use_try_from(42.0);
    println!("Infallible TryFrom: {:?}", result);

    demonstrate_from();
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
        let n: u32 = u32::from(100u8);
        assert_eq!(n, 100);
    }

    #[test]
    fn test_meters_from() {
        let m: Meters = Meters::from(5.0);
        assert_eq!(m, Meters(5.0));
    }
}

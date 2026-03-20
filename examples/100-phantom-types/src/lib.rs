#![allow(clippy::all)]
//! # Phantom Types — Type-Safe Units
//!
//! Use phantom type parameters to prevent mixing meters and seconds at compile time.
//! OCaml's abstract types map to Rust's `PhantomData<T>` marker.

use std::marker::PhantomData;
use std::ops::Add;

// ---------------------------------------------------------------------------
// Approach A: PhantomData marker (idiomatic Rust)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub struct Quantity<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

#[derive(Debug)]
pub struct Meters;
#[derive(Debug)]
pub struct Seconds;

impl<U> Quantity<U> {
    pub fn new(value: f64) -> Self {
        Quantity {
            value,
            _unit: PhantomData,
        }
    }

    pub fn value(&self) -> f64 {
        self.value
    }

    pub fn scale(&self, k: f64) -> Self {
        Quantity::new(k * self.value)
    }
}

impl<U> Add for Quantity<U> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Quantity::new(self.value + rhs.value)
    }
}

pub fn meters(v: f64) -> Quantity<Meters> {
    Quantity::new(v)
}
pub fn seconds(v: f64) -> Quantity<Seconds> {
    Quantity::new(v)
}

// ---------------------------------------------------------------------------
// Approach B: Newtype wrappers (simpler, no PhantomData)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MetersVal(pub f64);

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SecondsVal(pub f64);

impl Add for MetersVal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        MetersVal(self.0 + rhs.0)
    }
}

impl Add for SecondsVal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        SecondsVal(self.0 + rhs.0)
    }
}

// ---------------------------------------------------------------------------
// Approach C: Const generics (Rust-specific, experimental flavor)
// ---------------------------------------------------------------------------

// Using a string-based unit tag with const generics is nightly-only,
// but the concept shows Rust's direction for compile-time unit checking.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_same_units() {
        let d1 = meters(100.0);
        let d2 = meters(50.0);
        let total = d1 + d2;
        assert!((total.value() - 150.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_scale() {
        let t = seconds(3.0);
        let doubled = t.scale(2.0);
        assert!((doubled.value() - 6.0).abs() < f64::EPSILON);
    }

    // This should NOT compile — uncomment to verify:
    // #[test]
    // fn test_add_different_units_fails() {
    //     let d = meters(100.0);
    //     let t = seconds(5.0);
    //     let _ = d + t; // Compile error!
    // }

    #[test]
    fn test_newtype_add() {
        assert_eq!(MetersVal(10.0) + MetersVal(5.0), MetersVal(15.0));
    }

    #[test]
    fn test_phantom_zero_size() {
        assert_eq!(
            std::mem::size_of::<Quantity<Meters>>(),
            std::mem::size_of::<f64>()
        );
    }

    #[test]
    fn test_multiple_operations() {
        let d = meters(10.0) + meters(20.0);
        let scaled = d.scale(3.0);
        assert!((scaled.value() - 90.0).abs() < f64::EPSILON);
    }
}

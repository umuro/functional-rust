//! # Example 132: Phantom Units of Measure
//!
//! Tag numeric values with their unit of measure so the compiler prevents
//! accidental mixing of incompatible units — e.g., adding metres to feet
//! or passing a duration where a distance is expected.

use std::marker::PhantomData;
use std::ops::{Add, Div, Mul};

// ---------------------------------------------------------------------------
// Unit marker types (zero-sized; never stored in memory)
// ---------------------------------------------------------------------------

/// Approach 1: Phantom type units — simple marker structs.
/// Must be `Clone + Copy` so that `#[derive(Clone, Copy)]` on `Quantity<U>`
/// applies for all concrete unit types.
#[derive(Debug, Clone, Copy)]
pub struct Meters;
#[derive(Debug, Clone, Copy)]
pub struct Feet;
#[derive(Debug, Clone, Copy)]
pub struct Seconds;
#[derive(Debug, Clone, Copy)]
pub struct Kilograms;
#[derive(Debug, Clone, Copy)]
pub struct MetersPerSecond;
#[derive(Debug, Clone, Copy)]
pub struct NewtonSeconds; // impulse = kg·m/s

// ---------------------------------------------------------------------------
// Quantity<Unit> — wraps f64 + phantom unit tag
// ---------------------------------------------------------------------------

/// A numeric quantity tagged with a compile-time unit.
///
/// `PhantomData<Unit>` costs zero bytes at runtime but participates fully
/// in the type system, making `Quantity<Meters>` and `Quantity<Feet>`
/// distinct, incompatible types.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quantity<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

impl<U> Quantity<U> {
    /// Construct a quantity with the given value and unit inferred from context.
    pub fn new(value: f64) -> Self {
        Quantity {
            value,
            _unit: PhantomData,
        }
    }

    /// Extract the raw numeric value.
    pub fn value(self) -> f64 {
        self.value
    }

    /// Scale by a dimensionless factor (same unit).
    pub fn scale(self, factor: f64) -> Self {
        Quantity::new(self.value * factor)
    }
}

// ---------------------------------------------------------------------------
// Convenience constructors (mirror the OCaml `meters`, `seconds`, … helpers)
// ---------------------------------------------------------------------------

pub fn meters(v: f64) -> Quantity<Meters> {
    Quantity::new(v)
}
pub fn feet(v: f64) -> Quantity<Feet> {
    Quantity::new(v)
}
pub fn seconds(v: f64) -> Quantity<Seconds> {
    Quantity::new(v)
}
pub fn kilograms(v: f64) -> Quantity<Kilograms> {
    Quantity::new(v)
}

// ---------------------------------------------------------------------------
// Operator impls
// ---------------------------------------------------------------------------

/// Same-unit addition — only compiles when both sides share the same `Unit`.
impl<U> Add for Quantity<U> {
    type Output = Quantity<U>;
    fn add(self, rhs: Self) -> Self::Output {
        Quantity::new(self.value + rhs.value)
    }
}

/// Scalar multiplication — keeps the same unit.
impl<U> Mul<f64> for Quantity<U> {
    type Output = Quantity<U>;
    fn mul(self, rhs: f64) -> Self::Output {
        Quantity::new(self.value * rhs)
    }
}

// ---------------------------------------------------------------------------
// Physics relationships — type-checked dimensional analysis
// ---------------------------------------------------------------------------

/// distance / time → speed  (`Meters / Seconds = MetersPerSecond`)
impl Div<Quantity<Seconds>> for Quantity<Meters> {
    type Output = Quantity<MetersPerSecond>;
    fn div(self, rhs: Quantity<Seconds>) -> Self::Output {
        Quantity::new(self.value / rhs.value)
    }
}

/// momentum: mass × velocity → `Kilograms · MetersPerSecond = NewtonSeconds`
impl Mul<Quantity<MetersPerSecond>> for Quantity<Kilograms> {
    type Output = Quantity<NewtonSeconds>;
    fn mul(self, rhs: Quantity<MetersPerSecond>) -> Self::Output {
        Quantity::new(self.value * rhs.value)
    }
}

// ---------------------------------------------------------------------------
// Approach 2: Unit conversion (explicit, type-safe)
// ---------------------------------------------------------------------------

/// Convert feet to meters — produces a `Quantity<Meters>`.
/// The compiler forces you to call this explicitly; there is no implicit coercion.
pub fn feet_to_meters(q: Quantity<Feet>) -> Quantity<Meters> {
    Quantity::new(q.value * 0.3048)
}

/// Convert meters to feet.
pub fn meters_to_feet(q: Quantity<Meters>) -> Quantity<Feet> {
    Quantity::new(q.value / 0.3048)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- same-unit arithmetic ---

    #[test]
    fn test_add_same_unit() {
        let a = meters(3.0);
        let b = meters(4.0);
        assert_eq!((a + b).value(), 7.0);
    }

    #[test]
    fn test_scale() {
        let d = meters(5.0);
        assert_eq!(d.scale(2.0).value(), 10.0);
    }

    #[test]
    fn test_mul_scalar() {
        let d = seconds(4.0);
        assert_eq!((d * 3.0).value(), 12.0);
    }

    // --- dimensional analysis ---

    #[test]
    fn test_speed_from_distance_over_time() {
        let dist = meters(100.0);
        let time = seconds(10.0);
        let speed: Quantity<MetersPerSecond> = dist / time;
        assert_eq!(speed.value(), 10.0);
    }

    #[test]
    fn test_momentum_kg_times_mps() {
        let mass = kilograms(70.0);
        let speed: Quantity<MetersPerSecond> = Quantity::new(9.0);
        let momentum: Quantity<NewtonSeconds> = mass * speed;
        assert_eq!(momentum.value(), 630.0);
    }

    // --- unit conversion ---

    #[test]
    fn test_feet_to_meters() {
        let one_foot = feet(1.0);
        let in_meters = feet_to_meters(one_foot);
        let diff = (in_meters.value() - 0.3048).abs();
        assert!(diff < 1e-10, "expected ≈0.3048, got {}", in_meters.value());
    }

    #[test]
    fn test_meters_to_feet_roundtrip() {
        let original = meters(10.0);
        let roundtripped = feet_to_meters(meters_to_feet(original));
        let diff = (roundtripped.value() - original.value()).abs();
        assert!(diff < 1e-10);
    }

    #[test]
    fn test_feet_addition_stays_feet() {
        let a = feet(3.0);
        let b = feet(4.0);
        // result is Quantity<Feet>, not Quantity<Meters>
        let sum: Quantity<Feet> = a + b;
        assert_eq!(sum.value(), 7.0);
    }

    // --- zero / negative values ---

    #[test]
    fn test_zero_quantity() {
        let zero = meters(0.0);
        assert_eq!((zero + meters(5.0)).value(), 5.0);
    }

    #[test]
    fn test_negative_quantity() {
        let a = seconds(-3.0);
        let b = seconds(3.0);
        assert_eq!((a + b).value(), 0.0);
    }
}

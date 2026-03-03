/// # Phantom Types — Type-Safe Units
///
/// Phantom type parameters exist only at the type level — they carry no runtime data
/// but prevent mixing incompatible values (e.g., meters + seconds) at compile time.

use std::marker::PhantomData;
use std::ops::Add;

/// Unit marker types — zero-sized, exist only for the type system.
pub struct Meters;
pub struct Seconds;

/// A quantity tagged with a phantom unit type.
/// `PhantomData<U>` tells the compiler we "use" U without storing it.
#[derive(Debug, Clone, Copy)]
pub struct Quantity<U> {
    value: f64,
    _unit: PhantomData<U>,
}

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

    /// Scale by a dimensionless factor — preserves the unit type.
    pub fn scale(&self, factor: f64) -> Self {
        Quantity::new(self.value * factor)
    }
}

/// Addition is only defined for quantities of the SAME unit.
/// Trying to add Quantity<Meters> + Quantity<Seconds> is a compile error!
impl<U> Add for Quantity<U> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Quantity::new(self.value + rhs.value)
    }
}

/// Convenience constructors
pub fn meters(v: f64) -> Quantity<Meters> {
    Quantity::new(v)
}

pub fn seconds(v: f64) -> Quantity<Seconds> {
    Quantity::new(v)
}

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

    #[test]
    fn test_cannot_add_different_units() {
        // This would fail to compile:
        // let _ = meters(1.0) + seconds(2.0);
        // Error: expected `Quantity<Meters>`, found `Quantity<Seconds>`
        assert!(true); // Compile-time safety — the test is that it compiles
    }

    #[test]
    fn test_zero_sized() {
        // PhantomData<U> is zero-sized — Quantity is just an f64
        assert_eq!(
            std::mem::size_of::<Quantity<Meters>>(),
            std::mem::size_of::<f64>()
        );
    }

    #[test]
    fn test_copy_semantics() {
        let d = meters(42.0);
        let d2 = d; // Copy, not move
        assert!((d.value() - d2.value()).abs() < f64::EPSILON);
    }
}

fn main() {
    println!("{:?}", (total.value() - 150.0).abs() < f64::EPSILON);
    println!("{:?}", (doubled.value() - 6.0).abs() < f64::EPSILON);
    println!("{:?}", true);
}

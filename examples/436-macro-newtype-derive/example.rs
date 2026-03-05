// Deriving traits for newtypes in Rust
use std::fmt;
use std::ops::{Add, Sub, Mul, Deref, DerefMut};
use std::cmp::Ordering;

// Macro that generates common newtype impls
macro_rules! newtype {
    // Numeric newtype with arithmetic and display
    (numeric $name:ident($inner:ty) with unit $unit:expr) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
        struct $name($inner);

        impl $name {
            fn new(v: $inner) -> Self { $name(v) }
            fn value(self) -> $inner { self.0 }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{:.2}{}", self.0, $unit)
            }
        }

        impl Add for $name {
            type Output = Self;
            fn add(self, other: Self) -> Self { $name(self.0 + other.0) }
        }

        impl Sub for $name {
            type Output = Self;
            fn sub(self, other: Self) -> Self { $name(self.0 - other.0) }
        }

        impl Mul<$inner> for $name {
            type Output = Self;
            fn mul(self, s: $inner) -> Self { $name(self.0 * s) }
        }

        impl PartialEq<$inner> for $name {
            fn eq(&self, other: &$inner) -> bool { self.0 == *other }
        }
    };

    // Transparent wrapper with Deref
    (wrapper $name:ident($inner:ty)) => {
        #[derive(Debug, Clone, PartialEq)]
        struct $name($inner);

        impl $name {
            fn new(v: $inner) -> Self { $name(v) }
            fn into_inner(self) -> $inner { self.0 }
        }

        impl Deref for $name {
            type Target = $inner;
            fn deref(&self) -> &$inner { &self.0 }
        }

        impl DerefMut for $name {
            fn deref_mut(&mut self) -> &mut $inner { &mut self.0 }
        }

        impl fmt::Display for $name where $inner: fmt::Display {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<$inner> for $name {
            fn from(v: $inner) -> Self { $name(v) }
        }

        impl From<$name> for $inner {
            fn from(w: $name) -> $inner { w.0 }
        }
    };
}

// Generate newtypes
newtype!(numeric Meters(f64) with unit "m");
newtype!(numeric Kilograms(f64) with unit "kg");
newtype!(numeric Seconds(f64) with unit "s");
newtype!(wrapper Email(String));
newtype!(wrapper NonEmptyVec(Vec<i32>));

fn main() {
    let d1 = Meters::new(5.0);
    let d2 = Meters::new(3.0);
    println!("{} + {} = {}", d1, d2, d1 + d2);
    println!("{} - {} = {}", d1, d2, d1 - d2);
    println!("{} * 2 = {}", d1, d1 * 2.0);
    println!("d1 > d2: {}", d1 > d2);

    let w = Kilograms::new(70.0);
    let h = Meters::new(1.75);
    println!("
BMI: {:.1}", w.value() / (h.value() * h.value()));

    // Email wrapper
    let email = Email::new("user@example.com".to_string());
    println!("
Email: {}", email);
    println!("Contains @: {}", email.contains('@'));  // Deref to String
    let raw: String = email.into_inner();
    println!("Raw string: {}", raw);

    // NonEmptyVec
    let mut v = NonEmptyVec::new(vec![1, 2, 3]);
    v.push(4);  // DerefMut allows Vec methods
    println!("Vec: {:?}", *v);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_meters_arithmetic() {
        let a = Meters::new(5.0);
        let b = Meters::new(3.0);
        assert_eq!((a + b).value(), 8.0);
        assert_eq!((a - b).value(), 2.0);
    }

    #[test]
    fn test_email_deref() {
        let e = Email::new("test@test.com".to_string());
        assert!(e.contains('@'));
        assert!(e.ends_with(".com"));
    }
}

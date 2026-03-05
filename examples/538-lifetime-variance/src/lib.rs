//! Variance: Covariant, Contravariant, Invariant
//!
//! How subtyping propagates through type constructors.

use std::marker::PhantomData;

/// Covariant wrapper (like &'a T).
pub struct Covariant<'a, T> {
    _marker: PhantomData<&'a T>,
}

/// Invariant wrapper (like &'a mut T).
pub struct Invariant<'a, T> {
    _marker: PhantomData<&'a mut T>,
}

/// Covariant: longer lifetime can be used where shorter expected.
pub fn covariant_demo<'short>(s: &'short str) -> &'short str {
    let long: &'static str = "static";
    // 'static coerces to 'short — covariant
    long
}

/// Demonstrate covariance with Vec.
pub fn vec_covariance<'a>(v: Vec<&'static str>) -> Vec<&'a str> {
    // Vec<&'static T> can coerce to Vec<&'a T> for immutable use
    v
}

/// Cell<T> is invariant in T.
pub fn invariant_example() {
    use std::cell::Cell;
    let cell: Cell<i32> = Cell::new(5);
    cell.set(10);
    assert_eq!(cell.get(), 10);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_covariant_demo() {
        let local = String::from("local");
        let result = covariant_demo(&local);
        assert_eq!(result, "static");
    }

    #[test]
    fn test_vec_covariance() {
        let v: Vec<&'static str> = vec!["a", "b", "c"];
        let v2: Vec<&str> = vec_covariance(v);
        assert_eq!(v2.len(), 3);
    }

    #[test]
    fn test_invariant() {
        invariant_example();
    }
}

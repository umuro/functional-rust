//! # Monoid Laws
//!
//! A monoid is a semigroup with an identity element.
//! Laws:
//! - Left identity: mempty <> x = x
//! - Right identity: x <> mempty = x
//! - Associativity: (x <> y) <> z = x <> (y <> z)

/// Trait representing a monoid
pub trait Monoid: Sized {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

// Approach 1: Sum monoid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sum<T>(pub T);

impl Monoid for Sum<i32> {
    fn empty() -> Self { Sum(0) }
    fn combine(self, other: Self) -> Self { Sum(self.0 + other.0) }
}

impl Monoid for Sum<i64> {
    fn empty() -> Self { Sum(0) }
    fn combine(self, other: Self) -> Self { Sum(self.0 + other.0) }
}

// Approach 2: Product monoid
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Product<T>(pub T);

impl Monoid for Product<i32> {
    fn empty() -> Self { Product(1) }
    fn combine(self, other: Self) -> Self { Product(self.0 * other.0) }
}

// Approach 3: String monoid
impl Monoid for String {
    fn empty() -> Self { String::new() }
    fn combine(self, other: Self) -> Self { self + &other }
}

// Vec monoid
impl<T> Monoid for Vec<T> {
    fn empty() -> Self { Vec::new() }
    fn combine(mut self, mut other: Self) -> Self {
        self.append(&mut other);
        self
    }
}

// All/Any boolean monoids
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct All(pub bool);

impl Monoid for All {
    fn empty() -> Self { All(true) }
    fn combine(self, other: Self) -> Self { All(self.0 && other.0) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Any(pub bool);

impl Monoid for Any {
    fn empty() -> Self { Any(false) }
    fn combine(self, other: Self) -> Self { Any(self.0 || other.0) }
}

/// Verify left identity: empty <> x = x
pub fn verify_left_identity<M: Monoid + Clone + PartialEq>(x: M) -> bool {
    M::empty().combine(x.clone()) == x
}

/// Verify right identity: x <> empty = x
pub fn verify_right_identity<M: Monoid + Clone + PartialEq>(x: M) -> bool {
    x.clone().combine(M::empty()) == x
}

/// Verify associativity
pub fn verify_associativity<M: Monoid + Clone + PartialEq>(a: M, b: M, c: M) -> bool {
    let left = a.clone().combine(b.clone()).combine(c.clone());
    let right = a.combine(b.combine(c));
    left == right
}

/// Fold a collection using monoid
pub fn mconcat<M: Monoid>(items: impl IntoIterator<Item = M>) -> M {
    items.into_iter().fold(M::empty(), |acc, x| acc.combine(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_empty() {
        assert_eq!(Sum::<i32>::empty(), Sum(0));
    }

    #[test]
    fn test_sum_left_identity() {
        assert!(verify_left_identity(Sum(42)));
    }

    #[test]
    fn test_sum_right_identity() {
        assert!(verify_right_identity(Sum(42)));
    }

    #[test]
    fn test_sum_associativity() {
        assert!(verify_associativity(Sum(1), Sum(2), Sum(3)));
    }

    #[test]
    fn test_product_monoid() {
        assert_eq!(Product::<i32>::empty(), Product(1));
        assert!(verify_left_identity(Product(5)));
        assert!(verify_right_identity(Product(5)));
    }

    #[test]
    fn test_string_monoid() {
        assert!(verify_left_identity(String::from("hello")));
        assert!(verify_right_identity(String::from("world")));
    }

    #[test]
    fn test_vec_monoid() {
        assert!(verify_left_identity(vec![1, 2, 3]));
        assert!(verify_right_identity(vec![4, 5, 6]));
    }

    #[test]
    fn test_all_monoid() {
        assert_eq!(All::empty(), All(true));
        assert_eq!(All(true).combine(All(true)), All(true));
        assert_eq!(All(true).combine(All(false)), All(false));
    }

    #[test]
    fn test_any_monoid() {
        assert_eq!(Any::empty(), Any(false));
        assert_eq!(Any(false).combine(Any(true)), Any(true));
        assert_eq!(Any(false).combine(Any(false)), Any(false));
    }

    #[test]
    fn test_mconcat() {
        let nums = vec![Sum(1), Sum(2), Sum(3), Sum(4)];
        assert_eq!(mconcat(nums), Sum(10));
    }

    #[test]
    fn test_mconcat_empty() {
        let empty: Vec<Sum<i32>> = vec![];
        assert_eq!(mconcat(empty), Sum(0));
    }
}

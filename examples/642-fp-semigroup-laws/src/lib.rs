//! # Semigroup Laws
//!
//! A semigroup is a type with an associative binary operation.
//! Law: (a <> b) <> c = a <> (b <> c)

/// Trait representing a semigroup
pub trait Semigroup {
    fn combine(self, other: Self) -> Self;
}

// Approach 1: Sum semigroup for numbers
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sum<T>(pub T);

impl Semigroup for Sum<i32> {
    fn combine(self, other: Self) -> Self {
        Sum(self.0 + other.0)
    }
}

impl Semigroup for Sum<i64> {
    fn combine(self, other: Self) -> Self {
        Sum(self.0 + other.0)
    }
}

// Approach 2: Product semigroup
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Product<T>(pub T);

impl Semigroup for Product<i32> {
    fn combine(self, other: Self) -> Self {
        Product(self.0 * other.0)
    }
}

// Approach 3: String/Vec concatenation semigroup
impl Semigroup for String {
    fn combine(self, other: Self) -> Self {
        self + &other
    }
}

impl<T> Semigroup for Vec<T> {
    fn combine(mut self, mut other: Self) -> Self {
        self.append(&mut other);
        self
    }
}

// Max semigroup
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Max<T>(pub T);

impl<T: Ord> Semigroup for Max<T> {
    fn combine(self, other: Self) -> Self {
        if self.0 >= other.0 { self } else { other }
    }
}

// Min semigroup
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Min<T>(pub T);

impl<T: Ord> Semigroup for Min<T> {
    fn combine(self, other: Self) -> Self {
        if self.0 <= other.0 { self } else { other }
    }
}

/// Verify associativity law
pub fn verify_associativity<S: Semigroup + Clone + PartialEq>(a: S, b: S, c: S) -> bool {
    let left = a.clone().combine(b.clone()).combine(c.clone());
    let right = a.combine(b.combine(c));
    left == right
}

/// Combine multiple values using semigroup
pub fn sconcat<S: Semigroup + Clone>(first: S, rest: impl IntoIterator<Item = S>) -> S {
    rest.into_iter().fold(first, |acc, x| acc.combine(x))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_semigroup() {
        let a = Sum(1);
        let b = Sum(2);
        assert_eq!(a.combine(b), Sum(3));
    }

    #[test]
    fn test_sum_associativity() {
        assert!(verify_associativity(Sum(1), Sum(2), Sum(3)));
    }

    #[test]
    fn test_product_semigroup() {
        let a = Product(2);
        let b = Product(3);
        assert_eq!(a.combine(b), Product(6));
    }

    #[test]
    fn test_product_associativity() {
        assert!(verify_associativity(Product(2), Product(3), Product(4)));
    }

    #[test]
    fn test_string_semigroup() {
        let a = String::from("Hello, ");
        let b = String::from("World!");
        assert_eq!(a.combine(b), "Hello, World!");
    }

    #[test]
    fn test_string_associativity() {
        assert!(verify_associativity(
            String::from("a"),
            String::from("b"),
            String::from("c")
        ));
    }

    #[test]
    fn test_vec_semigroup() {
        let a = vec![1, 2];
        let b = vec![3, 4];
        assert_eq!(a.combine(b), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_max_semigroup() {
        assert_eq!(Max(5).combine(Max(3)), Max(5));
        assert_eq!(Max(2).combine(Max(7)), Max(7));
    }

    #[test]
    fn test_min_semigroup() {
        assert_eq!(Min(5).combine(Min(3)), Min(3));
        assert_eq!(Min(2).combine(Min(7)), Min(2));
    }

    #[test]
    fn test_sconcat() {
        let result = sconcat(Sum(1), vec![Sum(2), Sum(3), Sum(4)]);
        assert_eq!(result, Sum(10));
    }

    #[test]
    fn test_max_associativity() {
        assert!(verify_associativity(Max(1), Max(5), Max(3)));
    }
}

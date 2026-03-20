#![allow(clippy::all)]
// Monoid typeclass pattern using Rust traits.
// OCaml uses first-class modules to pass MONOID implementations.
// Rust uses traits with associated constants/methods and zero-sized marker types.

// ── Monoid trait ──────────────────────────────────────────────────────────────

/// A type with an identity element and an associative binary operation.
pub trait Monoid {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

// ── Idiomatic: use the trait bound directly ───────────────────────────────────

/// Fold a slice using its Monoid instance.
/// OCaml: `List.fold_left M.combine M.empty lst`
pub fn concat_all<T: Monoid>(items: impl IntoIterator<Item = T>) -> T {
    items.into_iter().fold(T::empty(), T::combine)
}

// ── Concrete Monoid instances ─────────────────────────────────────────────────

/// Integer addition monoid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sum(pub i64);

impl Monoid for Sum {
    fn empty() -> Self {
        Sum(0)
    }
    fn combine(self, other: Self) -> Self {
        Sum(self.0 + other.0)
    }
}

/// Integer multiplication monoid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Product(pub i64);

impl Monoid for Product {
    fn empty() -> Self {
        Product(1)
    }
    fn combine(self, other: Self) -> Self {
        Product(self.0 * other.0)
    }
}

/// String concatenation monoid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Concat(pub String);

impl Monoid for Concat {
    fn empty() -> Self {
        Concat(String::new())
    }
    fn combine(self, other: Self) -> Self {
        Concat(self.0 + &other.0)
    }
}

/// Boolean conjunction (all-true) monoid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct All(pub bool);

impl Monoid for All {
    fn empty() -> Self {
        All(true)
    }
    fn combine(self, other: Self) -> Self {
        All(self.0 && other.0)
    }
}

// ── Functional / recursive style ─────────────────────────────────────────────

/// Same operation written with explicit recursion, mirroring OCaml's `fold_left`.
pub fn concat_all_rec<T: Monoid + Clone>(items: &[T]) -> T {
    match items {
        [] => T::empty(),
        [head, rest @ ..] => head.clone().combine(concat_all_rec(rest)),
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // -- concat_all (idiomatic) ------------------------------------------------

    #[test]
    fn test_sum_empty() {
        let result = concat_all::<Sum>([]);
        assert_eq!(result, Sum(0));
    }

    #[test]
    fn test_sum_single() {
        assert_eq!(concat_all([Sum(42)]), Sum(42));
    }

    #[test]
    fn test_sum_multiple() {
        let result = concat_all([1, 2, 3, 4, 5].map(Sum));
        assert_eq!(result, Sum(15));
    }

    #[test]
    fn test_product_empty() {
        assert_eq!(concat_all::<Product>([]), Product(1));
    }

    #[test]
    fn test_product_multiple() {
        let result = concat_all([1, 2, 3, 4, 5].map(Product));
        assert_eq!(result, Product(120));
    }

    #[test]
    fn test_concat_strings() {
        let words = ["hello", " ", "world"].map(|s| Concat(s.to_owned()));
        assert_eq!(concat_all(words), Concat("hello world".to_owned()));
    }

    #[test]
    fn test_concat_empty() {
        assert_eq!(concat_all::<Concat>([]), Concat(String::new()));
    }

    #[test]
    fn test_all_all_true() {
        let result = concat_all([All(true), All(true), All(true)]);
        assert_eq!(result, All(true));
    }

    #[test]
    fn test_all_with_false() {
        let result = concat_all([All(true), All(true), All(false)]);
        assert_eq!(result, All(false));
    }

    // -- concat_all_rec (recursive) -------------------------------------------

    #[test]
    fn test_rec_sum_empty() {
        let result = concat_all_rec::<Sum>(&[]);
        assert_eq!(result, Sum(0));
    }

    #[test]
    fn test_rec_sum_multiple() {
        let items: Vec<Sum> = [1, 2, 3, 4, 5].map(Sum).to_vec();
        assert_eq!(concat_all_rec(&items), Sum(15));
    }

    #[test]
    fn test_rec_product_multiple() {
        let items: Vec<Product> = [1, 2, 3, 4, 5].map(Product).to_vec();
        assert_eq!(concat_all_rec(&items), Product(120));
    }

    #[test]
    fn test_rec_all_with_false() {
        let items = [All(true), All(false), All(true)];
        assert_eq!(concat_all_rec(&items), All(false));
    }
}

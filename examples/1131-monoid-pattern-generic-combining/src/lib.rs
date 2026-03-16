/// A trait representing a Monoid: an associative binary operation with an identity element.
///
/// In OCaml this is a module type (first-class module); in Rust it is a trait.
/// Any type that implements `Monoid` can be combined with `concat_all`.
pub trait Monoid {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

// ---------------------------------------------------------------------------
// Solution 1: Idiomatic Rust — fold over an iterator using the Monoid trait
// ---------------------------------------------------------------------------

/// Fold a collection of monoidal values into one using the identity and combine.
/// Mirrors OCaml's `List.fold_left M.combine M.empty lst`.
pub fn concat_all<M: Monoid>(items: impl IntoIterator<Item = M>) -> M {
    items.into_iter().fold(M::empty(), M::combine)
}

// ---------------------------------------------------------------------------
// Solution 2: Functional/recursive — closer to OCaml style
// ---------------------------------------------------------------------------

/// Recursive version: consume a `Vec` by pattern-matching head/tail.
pub fn concat_all_recursive<M: Monoid>(items: Vec<M>) -> M {
    fn go<M: Monoid>(acc: M, mut rest: Vec<M>) -> M {
        if rest.is_empty() {
            acc
        } else {
            let head = rest.remove(0);
            go(M::combine(acc, head), rest)
        }
    }
    go(M::empty(), items)
}

// ---------------------------------------------------------------------------
// Newtype wrappers — one per "module instance" in OCaml
// ---------------------------------------------------------------------------

/// Additive monoid over i32  (OCaml: module Sum)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sum(pub i32);

impl Monoid for Sum {
    fn empty() -> Self {
        Sum(0)
    }
    fn combine(self, other: Self) -> Self {
        Sum(self.0 + other.0)
    }
}

/// Multiplicative monoid over i32  (OCaml: module Product)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Product(pub i32);

impl Monoid for Product {
    fn empty() -> Self {
        Product(1)
    }
    fn combine(self, other: Self) -> Self {
        Product(self.0 * other.0)
    }
}

/// String-concatenation monoid  (OCaml: module Concat)
#[derive(Debug, Clone, PartialEq)]
pub struct Concat(pub String);

impl Monoid for Concat {
    fn empty() -> Self {
        Concat(String::new())
    }
    fn combine(self, other: Self) -> Self {
        Concat(self.0 + &other.0)
    }
}

/// Boolean-AND monoid  (OCaml: module All)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct All(pub bool);

impl Monoid for All {
    fn empty() -> Self {
        All(true)
    }
    fn combine(self, other: Self) -> Self {
        All(self.0 && other.0)
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // --- Sum ---

    #[test]
    fn test_sum_empty_list() {
        let result = concat_all::<Sum>([]);
        assert_eq!(result, Sum(0));
    }

    #[test]
    fn test_sum_single_element() {
        let result = concat_all([Sum(42)]);
        assert_eq!(result, Sum(42));
    }

    #[test]
    fn test_sum_multiple_elements() {
        let result = concat_all([Sum(1), Sum(2), Sum(3), Sum(4), Sum(5)]);
        assert_eq!(result, Sum(15));
    }

    #[test]
    fn test_sum_identity_law() {
        // combine(empty, x) == x
        let x = Sum(7);
        assert_eq!(Sum::empty().combine(x), x);
        // combine(x, empty) == x
        assert_eq!(x.combine(Sum::empty()), x);
    }

    // --- Product ---

    #[test]
    fn test_product_empty_list() {
        let result = concat_all::<Product>([]);
        assert_eq!(result, Product(1));
    }

    #[test]
    fn test_product_multiple_elements() {
        let result = concat_all([Product(1), Product(2), Product(3), Product(4), Product(5)]);
        assert_eq!(result, Product(120));
    }

    #[test]
    fn test_product_with_zero() {
        let result = concat_all([Product(3), Product(0), Product(5)]);
        assert_eq!(result, Product(0));
    }

    // --- Concat ---

    #[test]
    fn test_concat_empty_list() {
        let result = concat_all::<Concat>([]);
        assert_eq!(result, Concat(String::new()));
    }

    #[test]
    fn test_concat_multiple_strings() {
        let result = concat_all([
            Concat("hello".into()),
            Concat(" ".into()),
            Concat("world".into()),
        ]);
        assert_eq!(result, Concat("hello world".into()));
    }

    #[test]
    fn test_concat_single_string() {
        let result = concat_all([Concat("only".into())]);
        assert_eq!(result, Concat("only".into()));
    }

    // --- All ---

    #[test]
    fn test_all_empty_list() {
        // vacuously true
        let result = concat_all::<All>([]);
        assert_eq!(result, All(true));
    }

    #[test]
    fn test_all_all_true() {
        let result = concat_all([All(true), All(true), All(true)]);
        assert_eq!(result, All(true));
    }

    #[test]
    fn test_all_contains_false() {
        let result = concat_all([All(true), All(true), All(false)]);
        assert_eq!(result, All(false));
    }

    #[test]
    fn test_all_single_false() {
        let result = concat_all([All(false)]);
        assert_eq!(result, All(false));
    }

    // --- Recursive variant ---

    #[test]
    fn test_recursive_sum_matches_fold() {
        let fold_result = concat_all([Sum(1), Sum(2), Sum(3)]);
        let rec_result = concat_all_recursive(vec![Sum(1), Sum(2), Sum(3)]);
        assert_eq!(fold_result, rec_result);
    }

    #[test]
    fn test_recursive_empty_list() {
        let result = concat_all_recursive::<Sum>(vec![]);
        assert_eq!(result, Sum(0));
    }
}

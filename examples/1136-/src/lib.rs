#![allow(clippy::all)]
// ---------------------------------------------------------------------------
// Solution 1: Closure-based — direct mapping of OCaml first-class modules
//
// OCaml passes `(module M : MONOID with type t = a)` at the call site,
// which bundles an `empty` value and a `combine` function.
// This solution makes that structure explicit: the caller provides both pieces.
// ---------------------------------------------------------------------------

/// Fold a list using an explicit identity element and a combining function.
///
/// This mirrors the OCaml call site precisely:
///   `concat_all (module Sum) [1;2;3;4;5]`
/// becomes:
///   `concat_with(0, |a, b| a + b, [1,2,3,4,5])`
pub fn concat_with<T>(
    empty: T,
    combine: impl Fn(T, T) -> T,
    items: impl IntoIterator<Item = T>,
) -> T {
    items.into_iter().fold(empty, combine)
}

// ---------------------------------------------------------------------------
// Solution 2: Idiomatic Rust — trait with a blanket fold function
//
// The trait encodes the monoid laws as an interface constraint, and the
// compiler resolves the implementation statically (monomorphisation).
// This is zero-cost; no runtime dispatch.
// ---------------------------------------------------------------------------

pub trait Monoid: Sized {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

/// Fold a collection of monoidal values into one.
///
/// Equivalent to OCaml's `List.fold_left M.combine M.empty lst`.
pub fn concat_all<M: Monoid>(items: impl IntoIterator<Item = M>) -> M {
    items.into_iter().fold(M::empty(), M::combine)
}

// ---------------------------------------------------------------------------
// Solution 3: Standard-library monoids — Sum and Product built into Rust
//
// For numeric types, the standard library already provides monoid-like
// behaviour through `std::iter::Sum` and `std::iter::Product`, which are
// exactly the sum and product monoids over numeric types.
// ---------------------------------------------------------------------------

/// Sum a sequence of numbers using the stdlib monoid (zero identity, + combine).
pub fn sum_all<T: std::iter::Sum>(items: impl IntoIterator<Item = T>) -> T {
    items.into_iter().sum()
}

/// Multiply a sequence of numbers using the stdlib monoid (one identity, * combine).
pub fn product_all<T: std::iter::Product>(items: impl IntoIterator<Item = T>) -> T {
    items.into_iter().product()
}

// ---------------------------------------------------------------------------
// Newtype wrappers for the Monoid trait — one per OCaml "module instance"
//
// OCaml can have two modules with `type t = int` (Sum and Product) because
// modules are distinguished by name. Rust uses newtypes for the same purpose:
// coherence rules prevent two `impl Monoid for i32` in the same crate.
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
        // Moves `self.0` into a new String and appends `other.0`,
        // reusing the heap buffer of the left-hand side.
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

    // --- Solution 1: closure-based ---

    #[test]
    fn test_closure_sum() {
        let result = concat_with(0, |a, b| a + b, [1, 2, 3, 4, 5]);
        assert_eq!(result, 15);
    }

    #[test]
    fn test_closure_product() {
        let result = concat_with(1, |a, b| a * b, [1, 2, 3, 4, 5]);
        assert_eq!(result, 120);
    }

    #[test]
    fn test_closure_concat() {
        let result = concat_with(
            String::new(),
            |a, b| a + &b,
            ["hello".to_owned(), " ".to_owned(), "world".to_owned()],
        );
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_closure_all_with_false() {
        let result = concat_with(true, |a, b| a && b, [true, true, false]);
        assert_eq!(result, false);
    }

    #[test]
    fn test_closure_empty_list_returns_identity() {
        // The empty-list case returns the identity element unchanged.
        let sum = concat_with(0, |a, b| a + b, std::iter::empty::<i32>());
        assert_eq!(sum, 0);
        let product = concat_with(1, |a, b| a * b, std::iter::empty::<i32>());
        assert_eq!(product, 1);
    }

    // --- Solution 2: trait-based ---

    #[test]
    fn test_trait_sum_five_elements() {
        let result = concat_all([Sum(1), Sum(2), Sum(3), Sum(4), Sum(5)]);
        assert_eq!(result, Sum(15));
    }

    #[test]
    fn test_trait_product_five_elements() {
        let result = concat_all([Product(1), Product(2), Product(3), Product(4), Product(5)]);
        assert_eq!(result, Product(120));
    }

    #[test]
    fn test_trait_concat_strings() {
        let result = concat_all([
            Concat("hello".into()),
            Concat(" ".into()),
            Concat("world".into()),
        ]);
        assert_eq!(result, Concat("hello world".into()));
    }

    #[test]
    fn test_trait_all_contains_false() {
        let result = concat_all([All(true), All(true), All(false)]);
        assert_eq!(result, All(false));
    }

    #[test]
    fn test_trait_empty_list_identity() {
        assert_eq!(concat_all::<Sum>([]), Sum(0));
        assert_eq!(concat_all::<Product>([]), Product(1));
        assert_eq!(concat_all::<All>([]), All(true));
    }

    // --- Solution 3: std::iter::Sum / Product ---

    #[test]
    fn test_stdlib_sum_matches_closure() {
        let closure_result = concat_with(0, |a, b| a + b, [1, 2, 3, 4, 5]);
        let stdlib_result = sum_all([1, 2, 3, 4, 5]);
        assert_eq!(closure_result, stdlib_result);
    }

    #[test]
    fn test_stdlib_product_matches_closure() {
        let closure_result = concat_with(1, |a, b| a * b, [1, 2, 3, 4, 5]);
        let stdlib_result = product_all([1, 2, 3, 4, 5]);
        assert_eq!(closure_result, stdlib_result);
    }

    // --- Monoid laws ---

    #[test]
    fn test_left_identity_law() {
        // combine(empty, x) == x for all monoids
        let x = Sum(42);
        assert_eq!(Sum::empty().combine(x), x);
    }

    #[test]
    fn test_right_identity_law() {
        // combine(x, empty) == x for all monoids
        let x = Product(7);
        assert_eq!(x.combine(Product::empty()), x);
    }

    #[test]
    fn test_associativity_law() {
        // combine(combine(a, b), c) == combine(a, combine(b, c))
        let a = Sum(1);
        let b = Sum(2);
        let c = Sum(3);
        assert_eq!(
            a.combine(b).combine(c),
            Sum(1).combine(Sum(2).combine(Sum(3)))
        );
    }
}

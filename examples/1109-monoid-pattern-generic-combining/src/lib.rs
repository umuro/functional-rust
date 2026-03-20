#![allow(clippy::all)]
// 1109: Monoid Pattern — Generic Combining
//
// OCaml uses first-class modules (module types) to express the Monoid abstraction.
// Rust uses traits + newtype wrappers to achieve the same result.
//
// Key translation: OCaml's `module type MONOID` → Rust's `trait Monoid`
// Key insight: where OCaml passes different modules for the same type,
// Rust requires distinct newtypes (Sum vs Product over i32).

/// A Monoid: a type with an identity element and an associative binary operation.
///
/// Laws (not enforced by the type system, but expected by callers):
///   - Left identity:  combine(empty(), x) == x
///   - Right identity: combine(x, empty()) == x
///   - Associativity:  combine(combine(x, y), z) == combine(x, combine(y, z))
pub trait Monoid {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

/// Fold an iterable using the Monoid, starting from the identity element.
///
/// Mirrors OCaml: `let concat_all (module M : MONOID) lst = List.fold_left M.combine M.empty lst`
///
/// In OCaml the module is passed explicitly; in Rust the trait bound on `M` is inferred.
pub fn concat_all<M: Monoid>(items: impl IntoIterator<Item = M>) -> M {
    items.into_iter().fold(M::empty(), M::combine)
}

/// Recursive implementation — mirrors OCaml's structural recursion style.
/// Requires `Clone` because we pattern-match on a slice but `combine` consumes `self`.
pub fn concat_all_recursive<M: Monoid + Clone>(items: &[M]) -> M {
    match items {
        [] => M::empty(),
        [x] => x.clone(),
        [head, tail @ ..] => head.clone().combine(concat_all_recursive(tail)),
    }
}

// ---------------------------------------------------------------------------
// Newtype wrappers
//
// In OCaml, `module Sum` and `module Product` are separate modules for the
// same underlying type `int`. In Rust we cannot implement one trait twice for
// the same type, so we use newtypes — thin wrappers that give each behaviour
// its own distinct type identity.
// ---------------------------------------------------------------------------

/// Additive monoid over i32: identity = 0, combine = (+)
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

/// Multiplicative monoid over i32: identity = 1, combine = (*)
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

/// Conjunctive monoid over bool: identity = true, combine = (&&)
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

/// String concatenation monoid: identity = "", combine = (^)
///
/// `String` has only one natural Monoid instance so we implement directly,
/// no newtype needed.
impl Monoid for String {
    fn empty() -> Self {
        String::new()
    }
    fn combine(self, other: Self) -> Self {
        // `self + &other` avoids an extra allocation compared to format!
        self + &other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Sum ---

    #[test]
    fn test_sum_multiple() {
        let result = concat_all([1, 2, 3, 4, 5].map(Sum));
        assert_eq!(result, Sum(15));
    }

    #[test]
    fn test_sum_empty_returns_identity() {
        assert_eq!(concat_all(std::iter::empty::<Sum>()), Sum(0));
    }

    #[test]
    fn test_sum_single() {
        assert_eq!(concat_all([Sum(42)]), Sum(42));
    }

    #[test]
    fn test_sum_recursive_matches_iterative() {
        let items = [1, 2, 3, 4, 5].map(Sum);
        assert_eq!(concat_all_recursive(&items), concat_all(items));
    }

    // --- Product ---

    #[test]
    fn test_product_multiple() {
        let result = concat_all([1, 2, 3, 4, 5].map(Product));
        assert_eq!(result, Product(120));
    }

    #[test]
    fn test_product_empty_returns_identity() {
        assert_eq!(concat_all(std::iter::empty::<Product>()), Product(1));
    }

    #[test]
    fn test_product_single() {
        assert_eq!(concat_all([Product(7)]), Product(7));
    }

    // --- String concat ---

    #[test]
    fn test_concat_strings() {
        let words = ["hello", " ", "world"].map(str::to_string);
        assert_eq!(concat_all(words), "hello world");
    }

    #[test]
    fn test_concat_empty_returns_identity() {
        assert_eq!(concat_all(std::iter::empty::<String>()), "");
    }

    #[test]
    fn test_concat_single() {
        assert_eq!(concat_all(["hello".to_string()]), "hello");
    }

    // --- All (bool conjunction) ---

    #[test]
    fn test_all_all_true() {
        let result = concat_all([true, true, true].map(All));
        assert_eq!(result, All(true));
    }

    #[test]
    fn test_all_with_false() {
        let result = concat_all([true, true, false].map(All));
        assert_eq!(result, All(false));
    }

    #[test]
    fn test_all_empty_returns_identity() {
        assert_eq!(concat_all(std::iter::empty::<All>()), All(true));
    }

    // --- Monoid laws ---

    #[test]
    fn test_left_identity_law() {
        let x = Sum(5);
        assert_eq!(Sum::empty().combine(x), x);
    }

    #[test]
    fn test_right_identity_law() {
        let x = Product(5);
        assert_eq!(x.combine(Product::empty()), x);
    }

    #[test]
    fn test_associativity_law() {
        let (a, b, c) = (Sum(1), Sum(2), Sum(3));
        assert_eq!(a.combine(b).combine(c), Sum(1).combine(b.combine(c)));
    }
}

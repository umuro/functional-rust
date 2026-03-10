/// A monoid is a type with an associative binary operation and an identity element.
///
/// This mirrors OCaml's `module type MONOID = sig type t val empty : t val combine : t -> t -> t end`
/// using Rust's trait system instead of first-class modules.
pub trait Monoid {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

// --- Solution 1: Idiomatic Rust — iterator fold with trait bound ---
// Uses Iterator::fold, the direct analogue of OCaml's List.fold_left
pub fn concat_all<M: Monoid>(items: impl IntoIterator<Item = M>) -> M {
    items.into_iter().fold(M::empty(), M::combine)
}

// --- Solution 2: Recursive — closer to OCaml's fold_left unrolled ---
// Explicit recursion over a slice, mirroring OCaml pattern matching on lists.
// Requires Clone because we need to produce owned values from references.
pub fn concat_all_recursive<M: Monoid + Clone>(items: &[M]) -> M {
    match items {
        [] => M::empty(),
        [x] => x.clone(),
        [x, rest @ ..] => M::combine(x.clone(), concat_all_recursive(rest)),
    }
}

// --- Solution 3: reduce-based — avoids calling empty() when list is non-empty ---
// Uses Iterator::reduce, returning None for empty iterators.
pub fn concat_all_reduce<M: Monoid>(items: impl IntoIterator<Item = M>) -> Option<M> {
    items.into_iter().reduce(M::combine)
}

// --- Monoid instances ---

/// Additive monoid over i64 (identity: 0, operation: +)
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

/// Multiplicative monoid over i64 (identity: 1, operation: *)
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

/// String concatenation monoid (identity: "", operation: +)
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

/// Boolean AND monoid (identity: true, operation: &&)
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

/// Boolean OR monoid (identity: false, operation: ||)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Any(pub bool);

impl Monoid for Any {
    fn empty() -> Self {
        Any(false)
    }
    fn combine(self, other: Self) -> Self {
        Any(self.0 || other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- Sum tests ---

    #[test]
    fn sum_empty_list() {
        assert_eq!(concat_all::<Sum>(vec![]), Sum(0));
    }

    #[test]
    fn sum_single_element() {
        assert_eq!(concat_all(vec![Sum(42)]), Sum(42));
    }

    #[test]
    fn sum_multiple_elements() {
        assert_eq!(
            concat_all(vec![Sum(1), Sum(2), Sum(3), Sum(4), Sum(5)]),
            Sum(15)
        );
    }

    #[test]
    fn sum_with_negatives() {
        assert_eq!(concat_all(vec![Sum(-1), Sum(2), Sum(-3)]), Sum(-2));
    }

    // --- Product tests ---

    #[test]
    fn product_empty_list() {
        assert_eq!(concat_all::<Product>(vec![]), Product(1));
    }

    #[test]
    fn product_single_element() {
        assert_eq!(concat_all(vec![Product(7)]), Product(7));
    }

    #[test]
    fn product_multiple_elements() {
        assert_eq!(
            concat_all(vec![
                Product(1),
                Product(2),
                Product(3),
                Product(4),
                Product(5)
            ]),
            Product(120)
        );
    }

    #[test]
    fn product_with_zero() {
        assert_eq!(
            concat_all(vec![Product(5), Product(0), Product(3)]),
            Product(0)
        );
    }

    // --- Concat tests ---

    #[test]
    fn concat_empty_list() {
        assert_eq!(concat_all::<Concat>(vec![]), Concat(String::new()));
    }

    #[test]
    fn concat_single_element() {
        assert_eq!(
            concat_all(vec![Concat("hello".into())]),
            Concat("hello".into())
        );
    }

    #[test]
    fn concat_multiple_elements() {
        assert_eq!(
            concat_all(vec![
                Concat("hello".into()),
                Concat(" ".into()),
                Concat("world".into())
            ]),
            Concat("hello world".into())
        );
    }

    // --- All (boolean AND) tests ---

    #[test]
    fn all_empty_list() {
        assert_eq!(concat_all::<All>(vec![]), All(true));
    }

    #[test]
    fn all_true_elements() {
        assert_eq!(concat_all(vec![All(true), All(true), All(true)]), All(true));
    }

    #[test]
    fn all_with_false() {
        assert_eq!(
            concat_all(vec![All(true), All(true), All(false)]),
            All(false)
        );
    }

    // --- Any (boolean OR) tests ---

    #[test]
    fn any_empty_list() {
        assert_eq!(concat_all::<Any>(vec![]), Any(false));
    }

    #[test]
    fn any_all_false() {
        assert_eq!(concat_all(vec![Any(false), Any(false)]), Any(false));
    }

    #[test]
    fn any_with_true() {
        assert_eq!(
            concat_all(vec![Any(false), Any(true), Any(false)]),
            Any(true)
        );
    }

    // --- Recursive variant tests ---

    #[test]
    fn recursive_sum_empty() {
        assert_eq!(concat_all_recursive::<Sum>(&[]), Sum(0));
    }

    #[test]
    fn recursive_sum_multiple() {
        assert_eq!(
            concat_all_recursive(&[Sum(1), Sum(2), Sum(3), Sum(4), Sum(5)]),
            Sum(15)
        );
    }

    #[test]
    fn recursive_concat_multiple() {
        assert_eq!(
            concat_all_recursive(&[
                Concat("hello".into()),
                Concat(" ".into()),
                Concat("world".into())
            ]),
            Concat("hello world".into())
        );
    }

    // --- Reduce variant tests ---

    #[test]
    fn reduce_empty_returns_none() {
        assert_eq!(concat_all_reduce::<Sum>(vec![]), None);
    }

    #[test]
    fn reduce_nonempty_returns_some() {
        assert_eq!(
            concat_all_reduce(vec![Sum(1), Sum(2), Sum(3)]),
            Some(Sum(6))
        );
    }

    // --- Monoid law tests ---

    #[test]
    fn sum_identity_law() {
        // combine(empty, x) == x and combine(x, empty) == x
        let x = Sum(42);
        assert_eq!(Sum::combine(Sum::empty(), x), x);
        assert_eq!(Sum::combine(x, Sum::empty()), x);
    }

    #[test]
    fn sum_associativity_law() {
        // combine(combine(a, b), c) == combine(a, combine(b, c))
        let (a, b, c) = (Sum(1), Sum(2), Sum(3));
        assert_eq!(
            Sum::combine(Sum::combine(a, b), c),
            Sum::combine(a, Sum::combine(b, c))
        );
    }

    #[test]
    fn product_identity_law() {
        let x = Product(42);
        assert_eq!(Product::combine(Product::empty(), x), x);
        assert_eq!(Product::combine(x, Product::empty()), x);
    }

    #[test]
    fn product_associativity_law() {
        let (a, b, c) = (Product(2), Product(3), Product(4));
        assert_eq!(
            Product::combine(Product::combine(a, b), c),
            Product::combine(a, Product::combine(b, c))
        );
    }
}

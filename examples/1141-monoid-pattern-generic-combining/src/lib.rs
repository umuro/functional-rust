/// The Monoid typeclass — an associative binary operation with an identity element.
/// Maps directly to OCaml's `module type MONOID`.
pub trait Monoid {
    fn empty() -> Self;
    fn combine(self, other: Self) -> Self;
}

/// Idiomatic Rust: use the trait bound directly with a fold over a slice.
/// Equivalent to OCaml's `concat_all (module M) lst`.
pub fn concat_all<M: Monoid>(items: impl IntoIterator<Item = M>) -> M {
    items.into_iter().fold(M::empty(), M::combine)
}

/// Recursive implementation — closer to OCaml's `List.fold_left` pattern made explicit.
pub fn concat_all_recursive<M: Monoid + Clone>(items: &[M]) -> M {
    match items {
        [] => M::empty(),
        [x] => x.clone(),
        [x, rest @ ..] => x.clone().combine(concat_all_recursive(rest)),
    }
}

// --- Monoid instances ---

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Sum(pub i64);

impl Monoid for Sum {
    fn empty() -> Self {
        Sum(0)
    }
    fn combine(self, other: Self) -> Self {
        Sum(self.0 + other.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Product(pub i64);

impl Monoid for Product {
    fn empty() -> Self {
        Product(1)
    }
    fn combine(self, other: Self) -> Self {
        Product(self.0 * other.0)
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    // --- concat_all (idiomatic) ---

    #[test]
    fn test_sum_empty() {
        let result = concat_all::<Sum>([]);
        assert_eq!(result, Sum(0));
    }

    #[test]
    fn test_sum_multiple() {
        let result = concat_all([Sum(1), Sum(2), Sum(3), Sum(4), Sum(5)]);
        assert_eq!(result, Sum(15));
    }

    #[test]
    fn test_product_empty() {
        let result = concat_all::<Product>([]);
        assert_eq!(result, Product(1));
    }

    #[test]
    fn test_product_multiple() {
        let result = concat_all([Product(1), Product(2), Product(3), Product(4), Product(5)]);
        assert_eq!(result, Product(120));
    }

    #[test]
    fn test_concat_empty() {
        let result = concat_all::<Concat>([]);
        assert_eq!(result, Concat(String::new()));
    }

    #[test]
    fn test_concat_strings() {
        let result = concat_all([
            Concat("hello".into()),
            Concat(" ".into()),
            Concat("world".into()),
        ]);
        assert_eq!(result, Concat("hello world".into()));
    }

    #[test]
    fn test_all_true() {
        let result = concat_all([All(true), All(true), All(true)]);
        assert_eq!(result, All(true));
    }

    #[test]
    fn test_all_false_when_one_false() {
        let result = concat_all([All(true), All(true), All(false)]);
        assert_eq!(result, All(false));
    }

    // --- concat_all_recursive ---

    #[test]
    fn test_recursive_sum_empty() {
        let result = concat_all_recursive::<Sum>(&[]);
        assert_eq!(result, Sum(0));
    }

    #[test]
    fn test_recursive_sum_single() {
        let result = concat_all_recursive(&[Sum(42)]);
        assert_eq!(result, Sum(42));
    }

    #[test]
    fn test_recursive_product_multiple() {
        let result = concat_all_recursive(&[Product(2), Product(3), Product(5)]);
        assert_eq!(result, Product(30));
    }

    #[test]
    fn test_recursive_concat_strings() {
        let result = concat_all_recursive(&[
            Concat("foo".into()),
            Concat("bar".into()),
            Concat("baz".into()),
        ]);
        assert_eq!(result, Concat("foobarbaz".into()));
    }
}

#![allow(clippy::all)]
// A monoid viewed as a single-object category:
//   - One object: ()
//   - Morphisms: the monoid elements
//   - Composition: monoid append (associative)
//   - Identity: monoid empty

/// Trait encoding the Monoid abstraction (and therefore a single-object category).
pub trait Monoid {
    fn empty() -> Self;
    fn append(self, other: Self) -> Self;
}

// ── Concrete monoid instances ────────────────────────────────────────────────

/// String monoid: identity = "", operation = concatenation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringMonoid(pub String);

impl Monoid for StringMonoid {
    fn empty() -> Self {
        StringMonoid(String::new())
    }
    fn append(self, other: Self) -> Self {
        StringMonoid(self.0 + &other.0)
    }
}

/// Sum monoid: identity = 0, operation = addition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SumMonoid(pub i64);

impl Monoid for SumMonoid {
    fn empty() -> Self {
        SumMonoid(0)
    }
    fn append(self, other: Self) -> Self {
        SumMonoid(self.0 + other.0)
    }
}

/// Product monoid: identity = 1, operation = multiplication
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProductMonoid(pub i64);

impl Monoid for ProductMonoid {
    fn empty() -> Self {
        ProductMonoid(1)
    }
    fn append(self, other: Self) -> Self {
        ProductMonoid(self.0 * other.0)
    }
}

/// List (Vec) monoid: identity = [], operation = concatenation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListMonoid<T>(pub Vec<T>);

impl<T: Clone> Monoid for ListMonoid<T> {
    fn empty() -> Self {
        ListMonoid(Vec::new())
    }
    fn append(self, other: Self) -> Self {
        let mut v = self.0;
        v.extend(other.0);
        ListMonoid(v)
    }
}

// ── Monoid laws (as functions, not just assertions) ──────────────────────────

/// Left identity law: empty <> x == x
pub fn check_left_identity<M: Monoid + PartialEq + Clone>(x: M) -> bool {
    M::empty().append(x.clone()) == x
}

/// Right identity law: x <> empty == x
pub fn check_right_identity<M: Monoid + PartialEq + Clone>(x: M) -> bool {
    x.clone().append(M::empty()) == x
}

/// Associativity law: (x <> y) <> z == x <> (y <> z)
pub fn check_associativity<M: Monoid + PartialEq + Clone>(x: M, y: M, z: M) -> bool {
    x.clone().append(y.clone()).append(z.clone()) == x.append(y.append(z))
}

// ── Monoid as category: fold morphisms via composition ───────────────────────

/// Compose a sequence of morphisms (monoid elements) using fold_left —
/// mirrors `List.fold_left M.append M.empty ms` from OCaml.
pub fn compose_morphisms<M: Monoid>(morphisms: impl IntoIterator<Item = M>) -> M {
    morphisms
        .into_iter()
        .fold(M::empty(), |acc, m| acc.append(m))
}

// ── Recursive style (explicit fold) ─────────────────────────────────────────

/// Recursive version of compose_morphisms, closer to the OCaml recursive style.
pub fn compose_morphisms_recursive<M: Monoid + Clone>(morphisms: &[M]) -> M {
    match morphisms {
        [] => M::empty(),
        [x] => x.clone(),
        [head, rest @ ..] => head.clone().append(compose_morphisms_recursive(rest)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── StringMonoid ──────────────────────────────────────────────────────────

    #[test]
    fn test_string_left_identity() {
        assert!(check_left_identity(StringMonoid("hello".into())));
    }

    #[test]
    fn test_string_right_identity() {
        assert!(check_right_identity(StringMonoid("hello".into())));
    }

    #[test]
    fn test_string_associativity() {
        assert!(check_associativity(
            StringMonoid("a".into()),
            StringMonoid("b".into()),
            StringMonoid("c".into()),
        ));
    }

    #[test]
    fn test_string_compose_morphisms() {
        let words = vec![
            StringMonoid("hello".into()),
            StringMonoid(", ".into()),
            StringMonoid("world".into()),
            StringMonoid("!".into()),
        ];
        assert_eq!(
            compose_morphisms(words),
            StringMonoid("hello, world!".into())
        );
    }

    #[test]
    fn test_string_compose_empty() {
        let empty: Vec<StringMonoid> = vec![];
        assert_eq!(compose_morphisms(empty), StringMonoid(String::new()));
    }

    // ── SumMonoid ─────────────────────────────────────────────────────────────

    #[test]
    fn test_sum_identity() {
        assert!(check_left_identity(SumMonoid(42)));
        assert!(check_right_identity(SumMonoid(42)));
    }

    #[test]
    fn test_sum_associativity() {
        assert!(check_associativity(
            SumMonoid(1),
            SumMonoid(2),
            SumMonoid(3)
        ));
    }

    #[test]
    fn test_sum_compose() {
        let ms = vec![SumMonoid(1), SumMonoid(2), SumMonoid(3), SumMonoid(4)];
        assert_eq!(compose_morphisms(ms), SumMonoid(10));
    }

    // ── ProductMonoid ─────────────────────────────────────────────────────────

    #[test]
    fn test_product_identity() {
        assert!(check_left_identity(ProductMonoid(5)));
        assert!(check_right_identity(ProductMonoid(5)));
    }

    #[test]
    fn test_product_associativity() {
        assert!(check_associativity(
            ProductMonoid(2),
            ProductMonoid(3),
            ProductMonoid(4)
        ));
    }

    #[test]
    fn test_product_compose() {
        let ms = vec![ProductMonoid(2), ProductMonoid(3), ProductMonoid(4)];
        assert_eq!(compose_morphisms(ms), ProductMonoid(24));
    }

    // ── ListMonoid ────────────────────────────────────────────────────────────

    #[test]
    fn test_list_identity() {
        assert!(check_left_identity(ListMonoid(vec![1, 2, 3])));
        assert!(check_right_identity(ListMonoid(vec![1, 2, 3])));
    }

    #[test]
    fn test_list_associativity() {
        assert!(check_associativity(
            ListMonoid(vec![1]),
            ListMonoid(vec![2]),
            ListMonoid(vec![3]),
        ));
    }

    #[test]
    fn test_list_compose() {
        let ms = vec![
            ListMonoid(vec![1, 2]),
            ListMonoid(vec![3, 4]),
            ListMonoid(vec![5]),
        ];
        assert_eq!(compose_morphisms(ms), ListMonoid(vec![1, 2, 3, 4, 5]));
    }

    // ── Recursive compose ─────────────────────────────────────────────────────

    #[test]
    fn test_recursive_compose_empty() {
        let ms: Vec<SumMonoid> = vec![];
        assert_eq!(compose_morphisms_recursive(&ms), SumMonoid(0));
    }

    #[test]
    fn test_recursive_compose_single() {
        assert_eq!(compose_morphisms_recursive(&[SumMonoid(7)]), SumMonoid(7));
    }

    #[test]
    fn test_recursive_compose_multiple() {
        let ms = vec![SumMonoid(1), SumMonoid(2), SumMonoid(3)];
        assert_eq!(compose_morphisms_recursive(&ms), SumMonoid(6));
    }

    #[test]
    fn test_recursive_string() {
        let words = vec![
            StringMonoid("foo".into()),
            StringMonoid("bar".into()),
            StringMonoid("baz".into()),
        ];
        assert_eq!(
            compose_morphisms_recursive(&words),
            StringMonoid("foobarbaz".into())
        );
    }
}

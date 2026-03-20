#![allow(clippy::all)]
// A semigroup is a set with an associative binary operation.
// Like a monoid but without requiring an identity element.
// Weaker but more widely applicable.

/// A semigroup: a type with an associative binary operation.
///
/// Law: (a.append(b)).append(c) == a.append(b.append(c))
pub trait Semigroup {
    fn append(self, other: Self) -> Self;
}

// ── Concrete semigroup instances ──────────────────────────────────────────────

/// NonEmpty list semigroup — concatenation (note: no empty element!)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NonEmptyList<T>(pub Vec<T>);

impl<T: Clone> Semigroup for NonEmptyList<T> {
    fn append(self, other: Self) -> Self {
        let mut v = self.0;
        v.extend(other.0);
        NonEmptyList(v)
    }
}

/// Min semigroup — take the smaller of two values
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Min(pub i64);

impl Semigroup for Min {
    fn append(self, other: Self) -> Self {
        Min(self.0.min(other.0))
    }
}

/// Max semigroup — take the larger of two values
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Max(pub i64);

impl Semigroup for Max {
    fn append(self, other: Self) -> Self {
        Max(self.0.max(other.0))
    }
}

/// First semigroup — always keep the first value, discard the second
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct First<T>(pub T);

impl<T> Semigroup for First<T> {
    fn append(self, _other: Self) -> Self {
        self
    }
}

// ── Semigroup combinators ─────────────────────────────────────────────────────

/// Reduce a non-empty slice using a left fold (mirrors OCaml's `sconcat`).
/// Returns `None` for empty input — semigroup has no identity to fall back on.
pub fn sconcat<S: Semigroup + Clone>(items: &[S]) -> Option<S> {
    let (head, tail) = items.split_first()?;
    Some(
        tail.iter()
            .cloned()
            .fold(head.clone(), |acc, x| acc.append(x)),
    )
}

/// Recursive version of `sconcat` — right-to-left, equivalent by associativity.
pub fn sconcat_recursive<S: Semigroup + Clone>(items: &[S]) -> Option<S> {
    match items {
        [] => None,
        [x] => Some(x.clone()),
        [head, rest @ ..] => sconcat_recursive(rest).map(|tail| head.clone().append(tail)),
    }
}

/// Check the associativity law: (a ⊕ b) ⊕ c == a ⊕ (b ⊕ c)
pub fn check_associativity<S: Semigroup + PartialEq + Clone>(a: S, b: S, c: S) -> bool {
    a.clone().append(b.clone()).append(c.clone()) == a.append(b.append(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Min semigroup ──────────────────────────────────────────────────────────

    #[test]
    fn test_min_single_pair() {
        assert_eq!(Min(3).append(Min(7)), Min(3));
        assert_eq!(Min(7).append(Min(3)), Min(3));
    }

    #[test]
    fn test_min_associativity() {
        assert!(check_associativity(Min(3), Min(1), Min(4)));
    }

    #[test]
    fn test_min_sconcat() {
        let nums = [3, 1, 4, 1, 5, 9, 2, 6].map(Min);
        assert_eq!(sconcat(&nums), Some(Min(1)));
    }

    #[test]
    fn test_min_sconcat_empty() {
        let empty: &[Min] = &[];
        assert_eq!(sconcat(empty), None);
    }

    // ── Max semigroup ──────────────────────────────────────────────────────────

    #[test]
    fn test_max_single_pair() {
        assert_eq!(Max(3).append(Max(7)), Max(7));
        assert_eq!(Max(7).append(Max(3)), Max(7));
    }

    #[test]
    fn test_max_associativity() {
        assert!(check_associativity(Max(3), Max(1), Max(4)));
    }

    #[test]
    fn test_max_sconcat() {
        let nums = [3, 1, 4, 1, 5, 9, 2, 6].map(Max);
        assert_eq!(sconcat(&nums), Some(Max(9)));
    }

    #[test]
    fn test_max_sconcat_single() {
        assert_eq!(sconcat(&[Max(42)]), Some(Max(42)));
    }

    // ── First semigroup ────────────────────────────────────────────────────────

    #[test]
    fn test_first_keeps_left() {
        assert_eq!(First("hello").append(First("world")), First("hello"));
    }

    #[test]
    fn test_first_sconcat() {
        let words = [First("first"), First("second"), First("third")];
        assert_eq!(sconcat(&words), Some(First("first")));
    }

    #[test]
    fn test_first_associativity() {
        assert!(check_associativity(First("a"), First("b"), First("c")));
    }

    // ── NonEmptyList semigroup ─────────────────────────────────────────────────

    #[test]
    fn test_nonemptylist_append() {
        let a = NonEmptyList(vec![1, 2]);
        let b = NonEmptyList(vec![3, 4]);
        assert_eq!(a.append(b), NonEmptyList(vec![1, 2, 3, 4]));
    }

    #[test]
    fn test_nonemptylist_associativity() {
        assert!(check_associativity(
            NonEmptyList(vec![1]),
            NonEmptyList(vec![2]),
            NonEmptyList(vec![3]),
        ));
    }

    #[test]
    fn test_nonemptylist_sconcat() {
        let lists = [
            NonEmptyList(vec![1, 2]),
            NonEmptyList(vec![3]),
            NonEmptyList(vec![4, 5]),
        ];
        assert_eq!(sconcat(&lists), Some(NonEmptyList(vec![1, 2, 3, 4, 5])));
    }

    // ── Recursive sconcat ──────────────────────────────────────────────────────

    #[test]
    fn test_sconcat_recursive_min() {
        let nums = [3, 1, 4, 1, 5].map(Min);
        assert_eq!(sconcat_recursive(&nums), Some(Min(1)));
    }

    #[test]
    fn test_sconcat_recursive_first() {
        let words = [First("alpha"), First("beta"), First("gamma")];
        assert_eq!(sconcat_recursive(&words), Some(First("alpha")));
    }

    #[test]
    fn test_sconcat_recursive_empty() {
        let empty: &[Max] = &[];
        assert_eq!(sconcat_recursive(empty), None);
    }

    #[test]
    fn test_sconcat_and_recursive_agree() {
        // By associativity, both folds must give the same result
        let nums = [3, 1, 4, 1, 5, 9, 2, 6].map(Min);
        assert_eq!(sconcat(&nums), sconcat_recursive(&nums));
    }
}

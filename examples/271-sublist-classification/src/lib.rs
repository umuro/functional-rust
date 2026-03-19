#![allow(clippy::all)]
//! Sublist classification: determine whether two lists are equal,
//! one is a sublist of the other, or they are unequal.

#[derive(Debug, PartialEq, Eq)]
pub enum Relation {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

// Solution 1: Idiomatic Rust — uses slice windows for contiguous sublist check
pub fn classify_idiomatic<T: PartialEq>(a: &[T], b: &[T]) -> Relation {
    if a == b {
        Relation::Equal
    } else if is_sublist_idiomatic(a, b) {
        Relation::Sublist
    } else if is_sublist_idiomatic(b, a) {
        Relation::Superlist
    } else {
        Relation::Unequal
    }
}

// Returns true if `sub` appears as a contiguous subslice anywhere in `lst`
fn is_sublist_idiomatic<T: PartialEq>(sub: &[T], lst: &[T]) -> bool {
    if sub.is_empty() {
        return true;
    }
    lst.windows(sub.len()).any(|w| w == sub)
}

// Solution 2: Functional/recursive — mirrors the OCaml pattern-match style
pub fn classify_recursive<T: PartialEq>(a: &[T], b: &[T]) -> Relation {
    if a == b {
        Relation::Equal
    } else if is_sublist_recursive(a, b) {
        Relation::Sublist
    } else if is_sublist_recursive(b, a) {
        Relation::Superlist
    } else {
        Relation::Unequal
    }
}

fn starts_with<T: PartialEq>(lst: &[T], prefix: &[T]) -> bool {
    match (lst, prefix) {
        (_, []) => true,
        ([], _) => false,
        ([h1, t1 @ ..], [h2, t2 @ ..]) => h1 == h2 && starts_with(t1, t2),
    }
}

fn is_sublist_recursive<T: PartialEq>(sub: &[T], lst: &[T]) -> bool {
    match lst {
        [] => sub.is_empty(),
        [_, rest @ ..] => starts_with(lst, sub) || is_sublist_recursive(sub, rest),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use Relation::*;

    // --- idiomatic ---

    #[test]
    fn test_idiomatic_equal() {
        assert_eq!(classify_idiomatic(&[1, 2, 3], &[1, 2, 3]), Equal);
    }

    #[test]
    fn test_idiomatic_sublist() {
        assert_eq!(classify_idiomatic(&[1, 2, 3], &[0, 1, 2, 3, 4]), Sublist);
    }

    #[test]
    fn test_idiomatic_superlist() {
        assert_eq!(classify_idiomatic(&[0, 1, 2, 3, 4], &[1, 2, 3]), Superlist);
    }

    #[test]
    fn test_idiomatic_unequal() {
        assert_eq!(classify_idiomatic(&[1, 2, 3], &[4, 5, 6]), Unequal);
    }

    #[test]
    fn test_idiomatic_empty_sub_is_sublist() {
        assert_eq!(classify_idiomatic::<i32>(&[], &[1, 2, 3]), Sublist);
    }

    #[test]
    fn test_idiomatic_both_empty_equal() {
        assert_eq!(classify_idiomatic::<i32>(&[], &[]), Equal);
    }

    #[test]
    fn test_idiomatic_non_contiguous_not_sublist() {
        // [1,3] is NOT a contiguous sublist of [1,2,3]
        assert_eq!(classify_idiomatic(&[1, 3], &[1, 2, 3]), Unequal);
    }

    // --- recursive ---

    #[test]
    fn test_recursive_equal() {
        assert_eq!(classify_recursive(&[1, 2, 3], &[1, 2, 3]), Equal);
    }

    #[test]
    fn test_recursive_sublist() {
        assert_eq!(classify_recursive(&[1, 2, 3], &[0, 1, 2, 3, 4]), Sublist);
    }

    #[test]
    fn test_recursive_superlist() {
        assert_eq!(classify_recursive(&[0, 1, 2, 3, 4], &[1, 2, 3]), Superlist);
    }

    #[test]
    fn test_recursive_unequal() {
        assert_eq!(classify_recursive(&[1, 2, 3], &[4, 5, 6]), Unequal);
    }

    #[test]
    fn test_recursive_empty_sub_is_sublist() {
        assert_eq!(classify_recursive::<i32>(&[], &[1, 2, 3]), Sublist);
    }

    #[test]
    fn test_recursive_both_empty_equal() {
        assert_eq!(classify_recursive::<i32>(&[], &[]), Equal);
    }

    #[test]
    fn test_recursive_non_contiguous_not_sublist() {
        assert_eq!(classify_recursive(&[1, 3], &[1, 2, 3]), Unequal);
    }
}

//! # Palindrome Check
//! OCaml 99 Problems #6 — Check whether a list is a palindrome.

/// Idiomatic Rust: compare iterators forward and reversed.
/// Works on slices — no allocation needed thanks to `DoubleEndedIterator`.
pub fn is_palindrome<T: PartialEq>(list: &[T]) -> bool {
    // Slices give us O(1) indexed access; we only need to compare half
    let n = list.len();
    (0..n / 2).all(|i| list[i] == list[n - 1 - i])
}

/// Functional style: mirror the OCaml approach of comparing with reversed copy.
/// Requires `Clone` because we build a reversed `Vec` (like OCaml's `List.rev`).
pub fn is_palindrome_rev<T: PartialEq + Clone>(list: &[T]) -> bool {
    let reversed: Vec<T> = list.iter().rev().cloned().collect();
    list == reversed.as_slice()
}

/// Iterator-based: zip forward and backward iterators.
/// No allocation, lazy evaluation — the most "Rustic functional" approach.
pub fn is_palindrome_iter<T: PartialEq>(list: &[T]) -> bool {
    list.iter().eq(list.iter().rev())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_palindrome() {
        assert!(is_palindrome(&[1, 2, 3, 2, 1]));
        assert!(is_palindrome_rev(&[1, 2, 3, 2, 1]));
        assert!(is_palindrome_iter(&[1, 2, 3, 2, 1]));
    }

    #[test]
    fn test_not_palindrome() {
        assert!(!is_palindrome(&[1, 2, 3, 4]));
        assert!(!is_palindrome_rev(&[1, 2, 3, 4]));
        assert!(!is_palindrome_iter(&[1, 2, 3, 4]));
    }

    #[test]
    fn test_empty() {
        assert!(is_palindrome::<i32>(&[]));
        assert!(is_palindrome_rev::<i32>(&[]));
        assert!(is_palindrome_iter::<i32>(&[]));
    }

    #[test]
    fn test_single() {
        assert!(is_palindrome(&[42]));
        assert!(is_palindrome_rev(&[42]));
        assert!(is_palindrome_iter(&[42]));
    }

    #[test]
    fn test_even_palindrome() {
        assert!(is_palindrome(&[1, 2, 2, 1]));
        assert!(!is_palindrome(&[1, 2, 3, 1]));
    }

    #[test]
    fn test_strings() {
        let words: Vec<&str> = vec!["a", "b", "a"];
        assert!(is_palindrome(&words));
    }
}

// Example 095: DoubleEndedIterator
// Iterate from both ends simultaneously using .rev(), .next_back(), and symmetric traversal.

// === Approach 1: Idiomatic Rust using DoubleEndedIterator ===

/// Returns the last `n` elements in original order, without reversing the full collection.
pub fn take_last<T: Clone>(data: &[T], n: usize) -> Vec<T> {
    // .rev().take(n) consumes from the back, then .rev() restores original order.
    data.iter()
        .rev()
        .take(n)
        .cloned()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect()
}

/// Returns the last element using back-iteration (no index arithmetic).
pub fn last_element<T>(data: &[T]) -> Option<&T> {
    data.iter().next_back()
}

/// Returns `(first, last)` by consuming from both ends of the same iterator.
pub fn ends(data: &[i32]) -> Option<(i32, i32)> {
    let mut iter = data.iter().copied();
    let first = iter.next()?;
    // If there is only one element, first == last.
    let last = iter.next_back().unwrap_or(first);
    Some((first, last))
}

// === Approach 2: Algorithms using simultaneous front/back traversal ===

/// Palindrome check by comparing elements from both ends inward.
/// Zero allocation: `iter.next()` and `iter.next_back()` narrow the same slice view.
pub fn palindrome_check<T: PartialEq>(data: &[T]) -> bool {
    let mut iter = data.iter();
    loop {
        match (iter.next(), iter.next_back()) {
            (Some(a), Some(b)) => {
                if a != b {
                    return false;
                }
            }
            // Ends met in the middle (odd) or crossed (even) — done.
            _ => return true,
        }
    }
}

/// Interleave elements from the front and back, converging toward the middle.
/// e.g. [1,2,3,4,5] -> [1,5,2,4,3]
pub fn interleave_ends<T: Clone>(data: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(data.len());
    let mut iter = data.iter();
    loop {
        match iter.next() {
            None => break,
            Some(front) => {
                result.push(front.clone());
                match iter.next_back() {
                    // Skip when front and back are the same element (odd-length middle).
                    Some(back) if !std::ptr::eq(front, back) => result.push(back.clone()),
                    _ => {}
                }
            }
        }
    }
    result
}

/// Functional/recursive palindrome check — mirrors the OCaml array-index approach.
pub fn palindrome_check_recursive<T: PartialEq>(data: &[T]) -> bool {
    match data {
        [] | [_] => true,
        [first, rest @ .., last] => first == last && palindrome_check_recursive(rest),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- take_last ---

    #[test]
    fn test_take_last_empty() {
        let empty: &[i32] = &[];
        assert_eq!(take_last(empty, 3), Vec::<i32>::new());
    }

    #[test]
    fn test_take_last_fewer_than_n() {
        assert_eq!(take_last(&[1, 2], 5), vec![1, 2]);
    }

    #[test]
    fn test_take_last_exact() {
        assert_eq!(take_last(&[1, 2, 3, 4, 5], 3), vec![3, 4, 5]);
    }

    #[test]
    fn test_take_last_zero() {
        assert_eq!(take_last(&[1, 2, 3], 0), Vec::<i32>::new());
    }

    // --- last_element ---

    #[test]
    fn test_last_element_empty() {
        assert_eq!(last_element::<i32>(&[]), None);
    }

    #[test]
    fn test_last_element_single() {
        assert_eq!(last_element(&[42]), Some(&42));
    }

    #[test]
    fn test_last_element_multiple() {
        assert_eq!(last_element(&[1, 2, 3, 99]), Some(&99));
    }

    // --- ends ---

    #[test]
    fn test_ends_empty() {
        assert_eq!(ends(&[]), None);
    }

    #[test]
    fn test_ends_single() {
        assert_eq!(ends(&[7]), Some((7, 7)));
    }

    #[test]
    fn test_ends_multiple() {
        assert_eq!(ends(&[1, 2, 3, 4, 5]), Some((1, 5)));
    }

    // --- palindrome_check ---

    #[test]
    fn test_palindrome_empty() {
        assert!(palindrome_check::<i32>(&[]));
    }

    #[test]
    fn test_palindrome_single() {
        assert!(palindrome_check(&[1]));
    }

    #[test]
    fn test_palindrome_even_true() {
        assert!(palindrome_check(&[1, 2, 2, 1]));
    }

    #[test]
    fn test_palindrome_odd_true() {
        assert!(palindrome_check(&[1, 2, 3, 2, 1]));
    }

    #[test]
    fn test_palindrome_false() {
        assert!(!palindrome_check(&[1, 2, 3]));
    }

    #[test]
    fn test_palindrome_strings() {
        assert!(palindrome_check(&["a", "b", "b", "a"]));
        assert!(!palindrome_check(&["a", "b", "c"]));
    }

    // --- palindrome_check_recursive ---

    #[test]
    fn test_palindrome_recursive_true() {
        assert!(palindrome_check_recursive(&[1, 2, 3, 2, 1]));
    }

    #[test]
    fn test_palindrome_recursive_false() {
        assert!(!palindrome_check_recursive(&[1, 2, 3]));
    }

    // --- interleave_ends ---

    #[test]
    fn test_interleave_empty() {
        assert_eq!(interleave_ends::<i32>(&[]), Vec::<i32>::new());
    }

    #[test]
    fn test_interleave_single() {
        assert_eq!(interleave_ends(&[42]), vec![42]);
    }

    #[test]
    fn test_interleave_even() {
        assert_eq!(interleave_ends(&[1, 2, 3, 4]), vec![1, 4, 2, 3]);
    }

    #[test]
    fn test_interleave_odd() {
        assert_eq!(interleave_ends(&[1, 2, 3, 4, 5]), vec![1, 5, 2, 4, 3]);
    }
}

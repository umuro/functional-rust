//! # Slice Patterns — Pattern Matching on Slices
//!
//! Using slice patterns for elegant destructuring.

/// Match on slice length and contents
pub fn describe_slice(slice: &[i32]) -> &'static str {
    match slice {
        [] => "empty",
        [_] => "single element",
        [_, _] => "two elements",
        [_, _, _] => "three elements",
        [1, ..] => "starts with 1",
        [.., 9] => "ends with 9",
        [first, .., last] if first == last => "first equals last",
        _ => "other",
    }
}

/// Extract head and tail
pub fn head_tail<T>(slice: &[T]) -> Option<(&T, &[T])> {
    match slice {
        [head, tail @ ..] => Some((head, tail)),
        [] => None,
    }
}

/// Extract first two
pub fn first_two<T>(slice: &[T]) -> Option<(&T, &T)> {
    match slice {
        [a, b, ..] => Some((a, b)),
        _ => None,
    }
}

/// Check if palindrome
pub fn is_palindrome<T: Eq>(slice: &[T]) -> bool {
    match slice {
        [] | [_] => true,
        [first, middle @ .., last] => first == last && is_palindrome(middle),
    }
}

/// Sum with pattern matching
pub fn pattern_sum(nums: &[i32]) -> i32 {
    match nums {
        [] => 0,
        [single] => *single,
        [first, rest @ ..] => first + pattern_sum(rest),
    }
}

/// Find in sorted array
pub fn binary_search_pattern(arr: &[i32], target: i32) -> bool {
    match arr {
        [] => false,
        [single] => *single == target,
        _ => {
            let mid = arr.len() / 2;
            match arr[mid].cmp(&target) {
                std::cmp::Ordering::Equal => true,
                std::cmp::Ordering::Greater => binary_search_pattern(&arr[..mid], target),
                std::cmp::Ordering::Less => binary_search_pattern(&arr[mid + 1..], target),
            }
        }
    }
}

/// Mutable slice patterns
pub fn swap_first_last<T>(slice: &mut [T]) {
    if let [first, .., last] = slice {
        std::mem::swap(first, last);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_describe() {
        assert_eq!(describe_slice(&[]), "empty");
        assert_eq!(describe_slice(&[1]), "single element");
        assert_eq!(describe_slice(&[1, 2]), "two elements");
        assert_eq!(describe_slice(&[1, 2, 3]), "starts with 1");
        assert_eq!(describe_slice(&[2, 3, 9]), "ends with 9");
        assert_eq!(describe_slice(&[5, 3, 5]), "first equals last");
    }

    #[test]
    fn test_head_tail() {
        assert_eq!(head_tail(&[1, 2, 3]), Some((&1, &[2, 3][..])));
        assert_eq!(head_tail(&[1]), Some((&1, &[][..])));
        assert_eq!(head_tail::<i32>(&[]), None);
    }

    #[test]
    fn test_first_two() {
        assert_eq!(first_two(&[1, 2, 3]), Some((&1, &2)));
        assert_eq!(first_two(&[1]), None);
    }

    #[test]
    fn test_palindrome() {
        assert!(is_palindrome(&[1, 2, 1]));
        assert!(is_palindrome(&[1, 2, 2, 1]));
        assert!(!is_palindrome(&[1, 2, 3]));
        assert!(is_palindrome::<i32>(&[]));
    }

    #[test]
    fn test_pattern_sum() {
        assert_eq!(pattern_sum(&[1, 2, 3, 4, 5]), 15);
        assert_eq!(pattern_sum(&[]), 0);
    }

    #[test]
    fn test_binary_search() {
        let arr = [1, 3, 5, 7, 9, 11, 13];
        assert!(binary_search_pattern(&arr, 7));
        assert!(!binary_search_pattern(&arr, 6));
    }

    #[test]
    fn test_swap_first_last() {
        let mut arr = [1, 2, 3, 4, 5];
        swap_first_last(&mut arr);
        assert_eq!(arr, [5, 2, 3, 4, 1]);
    }
}

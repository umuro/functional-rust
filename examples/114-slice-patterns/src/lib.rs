// Example 114: Slice Patterns
//
// Rust can pattern match on slices: [first, rest @ ..], [a, b], [head, .., tail]
// Similar to OCaml's list patterns but on contiguous memory with exhaustiveness checking.

// --- Approach 1: Recursive sum and take using head/tail patterns ---

/// Sum all elements recursively, mirroring OCaml's `| x :: rest -> x + sum rest`.
pub fn sum(slice: &[i32]) -> i32 {
    match slice {
        [] => 0,
        [x, rest @ ..] => x + sum(rest),
    }
}

/// Take the first `n` elements, returning a new Vec.
pub fn take(n: usize, slice: &[i32]) -> Vec<i32> {
    match (n, slice) {
        (0, _) | (_, []) => vec![],
        (_, [x, rest @ ..]) => {
            let mut result = vec![*x];
            result.extend(take(n - 1, rest));
            result
        }
    }
}

// --- Approach 2: Structural shape matching ---

/// Describe the shape of a slice.
pub fn describe(slice: &[i32]) -> &'static str {
    match slice {
        [] => "empty",
        [_] => "singleton",
        [_, _] => "pair",
        _ => "many",
    }
}

/// Return the first and last elements, if they exist.
pub fn first_and_last(slice: &[i32]) -> Option<(i32, i32)> {
    match slice {
        [] => None,
        [only] => Some((*only, *only)),
        [head, .., tail] => Some((*head, *tail)),
    }
}

// --- Approach 3: Idiomatic iterator-based alternatives ---

/// Idiomatic Rust: sum with `.sum()`.
pub fn sum_idiomatic(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

/// Idiomatic Rust: take with `.take()`.
pub fn take_idiomatic(n: usize, slice: &[i32]) -> Vec<i32> {
    slice.iter().copied().take(n).collect()
}

/// Detect if a slice is sorted ascending — recursive with slice patterns.
/// Matches [a, b, ..] then recurses on the tail slice.
pub fn is_sorted_asc(slice: &[i32]) -> bool {
    match slice {
        [] | [_] => true,
        [a, b, ..] => a <= b && is_sorted_asc(&slice[1..]),
    }
}

/// Idiomatic alternative: sorted check with `.windows(2)`.
pub fn is_sorted_asc_idiomatic(slice: &[i32]) -> bool {
    slice.windows(2).all(|w| w[0] <= w[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- sum ---
    #[test]
    fn test_sum_empty() {
        assert_eq!(sum(&[]), 0);
    }

    #[test]
    fn test_sum_single() {
        assert_eq!(sum(&[42]), 42);
    }

    #[test]
    fn test_sum_multiple() {
        assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn test_sum_matches_idiomatic() {
        let data = [10, 20, 30];
        assert_eq!(sum(&data), sum_idiomatic(&data));
    }

    // --- take ---
    #[test]
    fn test_take_zero() {
        assert_eq!(take(0, &[1, 2, 3]), vec![]);
    }

    #[test]
    fn test_take_fewer_than_available() {
        assert_eq!(take(3, &[1, 2, 3, 4, 5]), vec![1, 2, 3]);
    }

    #[test]
    fn test_take_more_than_available() {
        assert_eq!(take(10, &[1, 2]), vec![1, 2]);
    }

    #[test]
    fn test_take_matches_idiomatic() {
        let data = [1, 2, 3, 4, 5];
        assert_eq!(take(3, &data), take_idiomatic(3, &data));
    }

    // --- describe ---
    #[test]
    fn test_describe_empty() {
        assert_eq!(describe(&[]), "empty");
    }

    #[test]
    fn test_describe_singleton() {
        assert_eq!(describe(&[99]), "singleton");
    }

    #[test]
    fn test_describe_pair() {
        assert_eq!(describe(&[1, 2]), "pair");
    }

    #[test]
    fn test_describe_many() {
        assert_eq!(describe(&[1, 2, 3]), "many");
        assert_eq!(describe(&[1, 2, 3, 4, 5]), "many");
    }

    // --- first_and_last ---
    #[test]
    fn test_first_and_last_empty() {
        assert_eq!(first_and_last(&[]), None);
    }

    #[test]
    fn test_first_and_last_single() {
        assert_eq!(first_and_last(&[7]), Some((7, 7)));
    }

    #[test]
    fn test_first_and_last_multiple() {
        assert_eq!(first_and_last(&[1, 2, 3, 4, 5]), Some((1, 5)));
    }

    #[test]
    fn test_first_and_last_pair() {
        assert_eq!(first_and_last(&[3, 9]), Some((3, 9)));
    }

    // --- is_sorted_asc ---
    #[test]
    fn test_sorted_empty() {
        assert!(is_sorted_asc(&[]));
        assert!(is_sorted_asc_idiomatic(&[]));
    }

    #[test]
    fn test_sorted_ascending() {
        assert!(is_sorted_asc(&[1, 2, 3, 4]));
        assert!(is_sorted_asc_idiomatic(&[1, 2, 3, 4]));
    }

    #[test]
    fn test_not_sorted() {
        assert!(!is_sorted_asc(&[1, 3, 2, 4]));
        assert!(!is_sorted_asc_idiomatic(&[1, 3, 2, 4]));
    }

    #[test]
    fn test_sorted_equal_elements() {
        assert!(is_sorted_asc(&[5, 5, 5]));
        assert!(is_sorted_asc_idiomatic(&[5, 5, 5]));
    }
}

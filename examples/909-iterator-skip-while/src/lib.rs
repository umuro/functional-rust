//! 265. Conditional Skipping with skip_while()
//!
//! `skip_while(pred)` discards elements from the front of an iterator until the predicate
//! first returns `false`, then yields *all* remaining elements — including later ones that
//! match the predicate. This "once it switches, it never switches back" behavior distinguishes
//! it from `filter()`.

/// Skip elements while they are less than `threshold`, returning the rest.
pub fn skip_less_than(slice: &[i32], threshold: i32) -> Vec<i32> {
    slice
        .iter()
        .copied()
        .skip_while(|&x| x < threshold)
        .collect()
}

/// Strip leading whitespace using `skip_while` on chars (returns owned String).
pub fn lstrip_chars(s: &str) -> String {
    s.chars().skip_while(|c| c.is_whitespace()).collect()
}

/// Remove leading zeros from a slice.
///
/// The trailing zero inside the sequence is preserved — skipping already stopped.
pub fn strip_leading_zeros(slice: &[i32]) -> Vec<i32> {
    slice.iter().copied().skip_while(|&x| x == 0).collect()
}

/// Recursive implementation matching the OCaml `skip_while` idiom.
///
/// OCaml pattern-matches on the list head; Rust does the same on a slice.
pub fn skip_while_recursive<T, F>(slice: &[T], pred: F) -> &[T]
where
    F: Fn(&T) -> bool,
{
    match slice {
        [] => &[],
        [head, rest @ ..] if pred(head) => skip_while_recursive(rest, pred),
        _ => slice,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_skip_less_than_typical() {
        let nums = [1, 2, 3, 4, 5, 4, 3, 2, 1];
        assert_eq!(skip_less_than(&nums, 4), vec![4, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_skip_less_than_all_skipped() {
        let nums = [1, 2, 3];
        assert_eq!(skip_less_than(&nums, 10), Vec::<i32>::new());
    }

    #[test]
    fn test_skip_less_than_none_skipped() {
        let nums = [5, 6, 7];
        assert_eq!(skip_less_than(&nums, 1), vec![5, 6, 7]);
    }

    #[test]
    fn test_strip_leading_zeros_keeps_inner_zero() {
        let data = [0, 0, 0, 1, 2, 3, 0, 4];
        assert_eq!(strip_leading_zeros(&data), vec![1, 2, 3, 0, 4]);
    }

    #[test]
    fn test_strip_leading_zeros_all_zeros() {
        assert_eq!(strip_leading_zeros(&[0, 0, 0]), Vec::<i32>::new());
    }

    #[test]
    fn test_strip_leading_zeros_no_leading() {
        assert_eq!(strip_leading_zeros(&[1, 0, 2]), vec![1, 0, 2]);
    }

    #[test]
    fn test_lstrip_chars_basic() {
        assert_eq!(lstrip_chars("   hello world"), "hello world");
    }

    #[test]
    fn test_lstrip_chars_no_leading_space() {
        assert_eq!(lstrip_chars("hello"), "hello");
    }

    #[test]
    fn test_lstrip_chars_all_spaces() {
        assert_eq!(lstrip_chars("   "), "");
    }

    #[test]
    fn test_skip_while_recursive_matches_iter_adapter() {
        let nums = [1i32, 2, 3, 4, 5, 4, 3, 2, 1];
        let expected = skip_less_than(&nums, 4);
        let got = skip_while_recursive(&nums, |&x| x < 4);
        assert_eq!(got, expected.as_slice());
    }

    #[test]
    fn test_skip_while_recursive_empty() {
        let empty: &[i32] = &[];
        assert_eq!(skip_while_recursive(empty, |_| true), &[] as &[i32]);
    }
}

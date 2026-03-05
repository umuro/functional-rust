//! 264. Conditional stopping with take_while()
//!
//! `take_while(pred)` yields elements until the predicate first returns false.
//! Unlike `filter`, it stops permanently at the first non-matching element —
//! making it the only viable option for infinite iterators.

/// Idiomatic Rust: use the built-in iterator adapter.
pub fn take_while_less_than(slice: &[i32], threshold: i32) -> Vec<i32> {
    slice
        .iter()
        .copied()
        .take_while(|&x| x < threshold)
        .collect()
}

/// Leading positives from a slice — stops at the first non-positive value.
pub fn leading_positives(slice: &[i32]) -> Vec<i32> {
    slice.iter().copied().take_while(|&x| x > 0).collect()
}

/// Returns all n where the n-th triangular number (n*(n+1)/2) is below `limit`.
/// Works on the infinite iterator `1u64..` — safe because take_while is lazy.
pub fn triangular_indices_below(limit: u64) -> Vec<u64> {
    (1u64..).take_while(|&n| n * (n + 1) / 2 < limit).collect()
}

/// Functional/recursive — mirrors the OCaml implementation directly.
pub fn take_while_rec<T, F>(slice: &[T], pred: F) -> Vec<T>
where
    T: Copy,
    F: Fn(T) -> bool,
{
    match slice {
        [] => vec![],
        [x, rest @ ..] => {
            if pred(*x) {
                let mut result = vec![*x];
                result.extend(take_while_rec(rest, pred));
                result
            } else {
                vec![]
            }
        }
    }
}

/// Leading alphabetic characters from a string.
pub fn leading_alpha(s: &str) -> String {
    s.chars().take_while(|c| c.is_alphabetic()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take_while_less_than_stops_at_threshold() {
        assert_eq!(
            take_while_less_than(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 5),
            vec![1, 2, 3, 4]
        );
    }

    #[test]
    fn test_take_while_less_than_empty_when_first_fails() {
        assert_eq!(take_while_less_than(&[5, 1, 2, 3], 5), vec![]);
    }

    #[test]
    fn test_take_while_less_than_all_match() {
        assert_eq!(take_while_less_than(&[1, 2, 3], 10), vec![1, 2, 3]);
    }

    #[test]
    fn test_take_while_less_than_empty_input() {
        assert_eq!(take_while_less_than(&[], 5), vec![]);
    }

    #[test]
    fn test_leading_positives_stops_at_negative() {
        assert_eq!(
            leading_positives(&[3, 1, 4, 1, -5, 9, -2, 6]),
            vec![3, 1, 4, 1]
        );
    }

    #[test]
    fn test_leading_positives_all_negative() {
        assert_eq!(leading_positives(&[-1, -2, -3]), vec![]);
    }

    #[test]
    fn test_triangular_indices_below_30() {
        assert_eq!(triangular_indices_below(30), vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_triangular_indices_below_1_is_empty() {
        assert_eq!(triangular_indices_below(1), vec![]);
    }

    #[test]
    fn test_recursive_mirrors_idiomatic() {
        let data = [1i32, 2, 3, 4, 5, 6];
        let idiomatic = take_while_less_than(&data, 4);
        let recursive = take_while_rec(&data, |x| x < 4);
        assert_eq!(idiomatic, recursive);
    }

    #[test]
    fn test_leading_alpha_stops_at_digit() {
        assert_eq!(leading_alpha("hello123world"), "hello".to_string());
    }

    #[test]
    fn test_leading_alpha_empty_string() {
        assert_eq!(leading_alpha(""), "".to_string());
    }

    #[test]
    fn test_does_not_resume_after_stop() {
        // Critical: take_while stops permanently — trailing 3 is NOT collected
        let nums = [1i32, 2, 3, 4, 5, 4, 3];
        assert_eq!(take_while_less_than(&nums, 4), vec![1, 2, 3]);
    }
}

#![allow(clippy::all)]
//! 260. Stateful accumulation with scan()
//!
//! `scan()` is like `fold` but emits each intermediate state as an iterator element.
//! The closure receives `&mut state` and the current item, returns `Some(value)` to
//! continue or `None` to stop early.

/// Running sum using scan — idiomatic Rust iterator approach.
pub fn running_sum(nums: &[i64]) -> Vec<i64> {
    nums.iter()
        .scan(0i64, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect()
}

/// Running product using scan.
pub fn running_product(nums: &[i64]) -> Vec<i64> {
    nums.iter()
        .scan(1i64, |state, &x| {
            *state *= x;
            Some(*state)
        })
        .collect()
}

/// Prefix sums up to (and not including) the first value that exceeds `limit`.
/// Returns `None` from the closure to stop the iterator early.
pub fn running_sum_until(nums: &[i64], limit: i64) -> Vec<i64> {
    nums.iter()
        .scan(0i64, |state, &x| {
            *state += x;
            if *state > limit {
                None
            } else {
                Some(*state)
            }
        })
        .collect()
}

/// Running maximum — tracks the largest value seen so far.
pub fn running_max(nums: &[i64]) -> Vec<i64> {
    nums.iter()
        .scan(i64::MIN, |state, &x| {
            *state = (*state).max(x);
            Some(*state)
        })
        .collect()
}

/// Generic scan: mirrors the OCaml `scan init f lst` function.
/// Threads `init` as mutable state, applies `f(state, item)` at each step,
/// and collects all intermediate states.
pub fn scan<T, F>(init: T, mut f: F, items: &[T]) -> Vec<T>
where
    T: Copy,
    F: FnMut(T, T) -> T,
{
    items
        .iter()
        .scan(init, |state, &x| {
            *state = f(*state, x);
            Some(*state)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_sum_empty() {
        assert_eq!(running_sum(&[]), Vec::<i64>::new());
    }

    #[test]
    fn test_running_sum_single() {
        assert_eq!(running_sum(&[42]), vec![42]);
    }

    #[test]
    fn test_running_sum_multiple() {
        assert_eq!(running_sum(&[1, 2, 3, 4, 5]), vec![1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_running_sum_with_negatives() {
        let transactions = [100, -30, 50, -80, 200];
        assert_eq!(running_sum(&transactions), vec![100, 70, 120, 40, 240]);
    }

    #[test]
    fn test_running_product_multiple() {
        assert_eq!(running_product(&[1, 2, 3, 4, 5]), vec![1, 2, 6, 24, 120]);
    }

    #[test]
    fn test_running_product_single() {
        assert_eq!(running_product(&[7]), vec![7]);
    }

    #[test]
    fn test_running_sum_until_terminates_early() {
        // 1, 3, 6 are <= 6; next would be 10 > 6, so stops
        assert_eq!(running_sum_until(&[1, 2, 3, 4, 5], 6), vec![1, 3, 6]);
    }

    #[test]
    fn test_running_sum_until_no_early_stop() {
        assert_eq!(running_sum_until(&[1, 2, 3], 100), vec![1, 3, 6]);
    }

    #[test]
    fn test_running_max() {
        assert_eq!(
            running_max(&[3, 1, 4, 1, 5, 9, 2, 6]),
            vec![3, 3, 4, 4, 5, 9, 9, 9]
        );
    }

    #[test]
    fn test_generic_scan_sum() {
        let result = scan(0i64, |a, b| a + b, &[1, 2, 3, 4, 5]);
        assert_eq!(result, vec![1, 3, 6, 10, 15]);
    }

    #[test]
    fn test_generic_scan_product() {
        let result = scan(1i64, |a, b| a * b, &[1, 2, 3, 4, 5]);
        assert_eq!(result, vec![1, 2, 6, 24, 120]);
    }
}

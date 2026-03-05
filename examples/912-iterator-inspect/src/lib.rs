//! 273. Debugging iterators with inspect()
//!
//! `inspect(f)` taps into an iterator pipeline with a side-effect closure,
//! passing each value through unchanged — the `.tap()` pattern from Haskell/RxJS.

use std::sync::atomic::{AtomicUsize, Ordering};

/// Collect squared even numbers while recording what passed through each stage.
/// Returns (all_seen, evens_seen, result).
pub fn inspect_pipeline(range: std::ops::RangeInclusive<i32>) -> (Vec<i32>, Vec<i32>, Vec<i32>) {
    let mut all_seen = Vec::new();
    let mut evens_seen = Vec::new();

    let result: Vec<i32> = range
        .inspect(|&x| all_seen.push(x))
        .filter(|x| x % 2 == 0)
        .inspect(|&x| evens_seen.push(x))
        .map(|x| x * x)
        .collect();

    (all_seen, evens_seen, result)
}

/// Count elements at each pipeline stage using atomics (safe across closures).
pub fn count_stages(
    range: std::ops::RangeInclusive<i32>,
    predicate: fn(&i32) -> bool,
) -> (usize, usize) {
    let count_in = AtomicUsize::new(0);
    let count_out = AtomicUsize::new(0);

    let _: Vec<i32> = range
        .inspect(|_| {
            count_in.fetch_add(1, Ordering::SeqCst);
        })
        .filter(predicate)
        .inspect(|_| {
            count_out.fetch_add(1, Ordering::SeqCst);
        })
        .collect();

    (
        count_in.load(Ordering::SeqCst),
        count_out.load(Ordering::SeqCst),
    )
}

/// Log and discard negative values; pass positives through.
/// Returns (warnings_logged, cleaned_values).
pub fn log_negatives(values: &[i32]) -> (Vec<i32>, Vec<i32>) {
    let mut warnings = Vec::new();

    let cleaned: Vec<i32> = values
        .iter()
        .copied()
        .inspect(|&x| {
            if x < 0 {
                warnings.push(x);
            }
        })
        .filter(|&x| x >= 0)
        .collect();

    (warnings, cleaned)
}

/// Demonstrate inspect for tracing through a multi-step transformation.
/// Returns the trace log and final result.
///
/// Multiple `inspect` closures that mutate shared state require `RefCell`
/// because each closure holds a `&mut` borrow for the lifetime of the chain,
/// and the borrow checker disallows two simultaneous `&mut` borrows.
pub fn trace_pipeline(items: &[&str]) -> (Vec<String>, Vec<String>) {
    use std::cell::RefCell;
    let trace = RefCell::new(Vec::new());

    let result: Vec<String> = items
        .iter()
        .copied()
        .inspect(|s| trace.borrow_mut().push(format!("raw:{s}")))
        .filter(|s| !s.is_empty())
        .inspect(|s| trace.borrow_mut().push(format!("non-empty:{s}")))
        .map(|s| s.to_uppercase())
        .inspect(|s| trace.borrow_mut().push(format!("upper:{s}")))
        .collect();

    (trace.into_inner(), result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inspect_pipeline_observes_without_altering() {
        let (all_seen, evens_seen, result) = inspect_pipeline(1..=6);
        assert_eq!(all_seen, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(evens_seen, vec![2, 4, 6]);
        assert_eq!(result, vec![4, 16, 36]);
    }

    #[test]
    fn test_inspect_pipeline_empty_range() {
        let (all_seen, evens_seen, result) = inspect_pipeline(1..=0);
        assert!(all_seen.is_empty());
        assert!(evens_seen.is_empty());
        assert!(result.is_empty());
    }

    #[test]
    fn test_count_stages_tracks_filter_reduction() {
        let (total_in, total_out) = count_stages(1..=20, |x| x % 3 == 0);
        assert_eq!(total_in, 20);
        assert_eq!(total_out, 6);
    }

    #[test]
    fn test_count_stages_all_pass_filter() {
        let (total_in, total_out) = count_stages(1..=5, |_| true);
        assert_eq!(total_in, 5);
        assert_eq!(total_out, 5);
    }

    #[test]
    fn test_log_negatives_separates_correctly() {
        let (warnings, cleaned) = log_negatives(&[-1, 2, -3, 4, 5]);
        assert_eq!(warnings, vec![-1, -3]);
        assert_eq!(cleaned, vec![2, 4, 5]);
    }

    #[test]
    fn test_log_negatives_all_positive() {
        let (warnings, cleaned) = log_negatives(&[1, 2, 3]);
        assert!(warnings.is_empty());
        assert_eq!(cleaned, vec![1, 2, 3]);
    }

    #[test]
    fn test_trace_pipeline_records_each_stage() {
        let (trace, result) = trace_pipeline(&["hello", "", "world"]);
        assert_eq!(result, vec!["HELLO", "WORLD"]);
        assert_eq!(trace.iter().filter(|t| t.starts_with("raw:")).count(), 3);
        assert_eq!(
            trace.iter().filter(|t| t.starts_with("non-empty:")).count(),
            2
        );
        assert_eq!(trace.iter().filter(|t| t.starts_with("upper:")).count(), 2);
    }

    #[test]
    fn test_inspect_does_not_consume_values() {
        let result: Vec<i32> = (1..=5).inspect(|_| {}).map(|x| x * 2).collect();
        assert_eq!(result, vec![2, 4, 6, 8, 10]);
    }
}

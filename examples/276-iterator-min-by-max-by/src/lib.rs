#![allow(clippy::all)]
//! 276. Custom comparison min_by() and max_by()
//!
//! `min_by(cmp)` and `max_by(cmp)` take a `Fn(&A, &A) -> Ordering` comparator.

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    #[test]
    fn test_min_by_float() {
        let floats = [3.0f64, 1.0, 2.0];
        let min = floats
            .iter()
            .copied()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        assert_eq!(min, Some(1.0));
    }

    #[test]
    fn test_max_by_reversed() {
        let nums = [1i32, 5, 3, 2, 4];
        let max = nums.iter().min_by(|a, b| b.cmp(a));
        assert_eq!(max, Some(&5));
    }

    #[test]
    fn test_min_by_multi_key() {
        let words = ["bb", "aa", "c"];
        let min = words
            .iter()
            .min_by(|a, b| a.len().cmp(&b.len()).then_with(|| a.cmp(b)));
        assert_eq!(min, Some(&"c"));
    }
}

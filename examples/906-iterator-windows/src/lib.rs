//! 262. Sliding Windows over Slices
//!
//! `windows(n)` yields overlapping sub-slices of length `n`, zero-copy.
//! Each window shares memory with the original slice — no allocation at each step.

/// Compute moving averages over a slice using a window of size `k`.
///
/// Returns one average per window. Slice borrows the original data — no copies.
pub fn moving_average(data: &[i32], k: usize) -> Vec<f64> {
    data.windows(k)
        .map(|w| w.iter().sum::<i32>() as f64 / k as f64)
        .collect()
}

/// Return true if every consecutive pair is strictly increasing.
pub fn is_strictly_increasing(data: &[i32]) -> bool {
    data.windows(2).all(|w| w[0] < w[1])
}

/// Find indices of local maxima: elements strictly greater than both neighbours.
///
/// Returns the index into the *original* slice (window index + 1).
pub fn local_maxima(data: &[i32]) -> Vec<usize> {
    data.windows(3)
        .enumerate()
        .filter(|(_, w)| w[1] > w[0] && w[1] > w[2])
        .map(|(i, _)| i + 1) // +1: window index is the left neighbour's index
        .collect()
}

/// Extract all bigrams (consecutive pairs) from a slice of items.
pub fn bigrams<T>(data: &[T]) -> Vec<(&T, &T)> {
    data.windows(2).map(|w| (&w[0], &w[1])).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moving_average_basic() {
        let data = [1, 2, 3, 4, 5];
        let avgs = moving_average(&data, 3);
        assert_eq!(avgs, vec![2.0, 3.0, 4.0]);
    }

    #[test]
    fn test_moving_average_window_equals_len() {
        let data = [1, 2, 3];
        let avgs = moving_average(&data, 3);
        assert_eq!(avgs, vec![2.0]);
    }

    #[test]
    fn test_moving_average_window_larger_than_slice() {
        let data = [1, 2];
        let avgs = moving_average(&data, 5);
        assert!(avgs.is_empty());
    }

    #[test]
    fn test_is_strictly_increasing_true() {
        assert!(is_strictly_increasing(&[1, 2, 3, 4, 5]));
    }

    #[test]
    fn test_is_strictly_increasing_false() {
        assert!(!is_strictly_increasing(&[1, 2, 2, 3]));
        assert!(!is_strictly_increasing(&[5, 4, 3]));
    }

    #[test]
    fn test_is_strictly_increasing_single() {
        // A single-element slice has no pairs → vacuously true
        assert!(is_strictly_increasing(&[42]));
    }

    #[test]
    fn test_local_maxima() {
        let signal = [1, 3, 2, 5, 4, 6, 2];
        let peaks = local_maxima(&signal);
        // index 1 (value 3): 1 < 3 > 2 ✓
        // index 3 (value 5): 2 < 5 > 4 ✓
        // index 5 (value 6): 4 < 6 > 2 ✓
        assert_eq!(peaks, vec![1, 3, 5]);
    }

    #[test]
    fn test_local_maxima_flat() {
        let signal = [1, 1, 1, 1];
        assert!(local_maxima(&signal).is_empty());
    }

    #[test]
    fn test_bigrams() {
        let words = ["the", "cat", "sat"];
        let pairs = bigrams(&words);
        assert_eq!(pairs, vec![(&"the", &"cat"), (&"cat", &"sat")]);
    }

    #[test]
    fn test_bigrams_empty() {
        let empty: &[i32] = &[];
        assert!(bigrams(empty).is_empty());
    }

    #[test]
    fn test_window_count() {
        // L=5, n=3 → L - n + 1 = 3 windows
        let data = [10, 20, 30, 40, 50];
        assert_eq!(data.windows(3).count(), 3);
    }
}

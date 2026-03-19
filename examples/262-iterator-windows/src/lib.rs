//! 262. Sliding windows over slices
//!
//! `windows(n)` yields overlapping sub-slices of length `n`, zero-copy.

#[cfg(test)]
mod tests {
    #[test]
    fn test_windows_count() {
        let data = [1i32, 2, 3, 4, 5];
        assert_eq!(data.windows(3).count(), 3);
    }

    #[test]
    fn test_windows_moving_avg() {
        let data = [1i32, 2, 3, 4, 5];
        let avgs: Vec<f64> = data
            .windows(2)
            .map(|w| w.iter().sum::<i32>() as f64 / 2.0)
            .collect();
        assert_eq!(avgs, vec![1.5, 2.5, 3.5, 4.5]);
    }

    #[test]
    fn test_windows_sorted_check() {
        let sorted = [1i32, 2, 3, 4];
        assert!(sorted.windows(2).all(|w| w[0] <= w[1]));
        let unsorted = [1i32, 3, 2, 4];
        assert!(!unsorted.windows(2).all(|w| w[0] <= w[1]));
    }
}

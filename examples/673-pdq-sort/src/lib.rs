//! # PDQsort (Pattern-Defeating Quicksort)
//!
//! Rust's standard unstable sort. Hybrid algorithm optimized for real-world data.

/// Simplified pdqsort concept
pub fn pdqsort<T: Ord>(arr: &mut [T]) {
    // In practice, Rust's slice::sort_unstable is pdqsort
    arr.sort_unstable();
}

/// Partition with block partitioning concept
pub fn block_partition<T: Ord>(arr: &mut [T]) -> usize {
    if arr.len() <= 1 { return 0; }
    let pivot = arr.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if arr[j] <= arr[pivot] { arr.swap(i, j); i += 1; }
    }
    arr.swap(i, pivot);
    i
}

/// Check if array is already sorted (pattern detection)
pub fn is_sorted<T: Ord>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

/// Check for reverse-sorted pattern
pub fn is_reverse_sorted<T: Ord>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] >= w[1])
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pdqsort() {
        let mut arr = vec![5, 2, 8, 1, 9, 3];
        pdqsort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 8, 9]);
    }
    
    #[test]
    fn test_pattern_detection() {
        assert!(is_sorted(&[1, 2, 3, 4, 5]));
        assert!(is_reverse_sorted(&[5, 4, 3, 2, 1]));
    }
}

#![allow(clippy::all)]
//! # Algorithm Complexity Guide
//!
//! Reference for common algorithm complexities.

/// O(1) - Constant time
pub fn constant_time_example(arr: &[i32]) -> Option<i32> {
    arr.first().copied()
}

/// O(log n) - Logarithmic time
pub fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    arr.binary_search(&target).ok()
}

/// O(n) - Linear time
pub fn linear_search(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

/// O(n log n) - Linearithmic time
pub fn merge_sort(mut arr: Vec<i32>) -> Vec<i32> {
    if arr.len() <= 1 {
        return arr;
    }
    let mid = arr.len() / 2;
    let right = merge_sort(arr.split_off(mid));
    let left = merge_sort(arr);
    merge(left, right)
}

fn merge(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(a.len() + b.len());
    let (mut i, mut j) = (0, 0);
    while i < a.len() && j < b.len() {
        if a[i] <= b[j] {
            result.push(a[i]);
            i += 1;
        } else {
            result.push(b[j]);
            j += 1;
        }
    }
    result.extend_from_slice(&a[i..]);
    result.extend_from_slice(&b[j..]);
    result
}

/// O(n²) - Quadratic time
pub fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant() {
        assert_eq!(constant_time_example(&[1, 2, 3]), Some(1));
    }
    #[test]
    fn test_binary() {
        assert_eq!(binary_search(&[1, 2, 3, 4, 5], 3), Some(2));
    }
    #[test]
    fn test_linear() {
        assert_eq!(linear_search(&[1, 2, 3, 4, 5], 3), Some(2));
    }
    #[test]
    fn test_merge() {
        assert_eq!(merge_sort(vec![3, 1, 4, 1, 5, 9]), vec![1, 1, 3, 4, 5, 9]);
    }
}

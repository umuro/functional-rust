//! # Timsort
//!
//! Hybrid stable sort combining merge sort and insertion sort.
//! Time: O(n log n), Space: O(n), Stable: Yes

const MIN_RUN: usize = 32;

pub fn timsort<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();
    for start in (0..n).step_by(MIN_RUN) {
        let end = (start + MIN_RUN).min(n);
        insertion_sort(&mut arr[start..end]);
    }
    
    let mut size = MIN_RUN;
    while size < n {
        for left in (0..n).step_by(2 * size) {
            let mid = (left + size).min(n);
            let right = (left + 2 * size).min(n);
            if mid < right { merge(&mut arr[left..right], mid - left); }
        }
        size *= 2;
    }
}

fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] { arr.swap(j - 1, j); j -= 1; }
    }
}

fn merge<T: Ord + Clone>(arr: &mut [T], mid: usize) {
    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();
    let (mut i, mut j, mut k) = (0, 0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] { arr[k] = left[i].clone(); i += 1; }
        else { arr[k] = right[j].clone(); j += 1; }
        k += 1;
    }
    while i < left.len() { arr[k] = left[i].clone(); i += 1; k += 1; }
    while j < right.len() { arr[k] = right[j].clone(); j += 1; k += 1; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timsort() {
        let mut arr = vec![5, 2, 8, 1, 9, 3, 7, 4, 6];
        timsort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_large() {
        let mut arr: Vec<i32> = (0..1000).rev().collect();
        timsort(&mut arr);
        assert!(arr.windows(2).all(|w| w[0] <= w[1]));
    }
}

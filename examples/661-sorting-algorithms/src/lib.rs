//! # Sorting Algorithms Overview
//!
//! Comparison of common sorting algorithms with their trade-offs.

/// Bubble Sort: O(n²) time, O(1) space
pub fn bubble_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

/// Insertion Sort: O(n²) time, O(1) space - good for small/nearly sorted
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Selection Sort: O(n²) time, O(1) space
pub fn selection_sort<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min_idx = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        arr.swap(i, min_idx);
    }
}

/// Merge Sort: O(n log n) time, O(n) space - stable
pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 { return; }
    
    let mid = len / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    
    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();
    
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            arr[k] = left[i].clone();
            i += 1;
        } else {
            arr[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }
    
    while i < left.len() {
        arr[k] = left[i].clone();
        i += 1;
        k += 1;
    }
    
    while j < right.len() {
        arr[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

/// Quick Sort: O(n log n) average, O(n²) worst, O(log n) space
pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 { return; }
    
    let pivot_idx = partition(arr);
    quick_sort(&mut arr[..pivot_idx]);
    quick_sort(&mut arr[pivot_idx + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_idx = len - 1;
    let mut i = 0;
    
    for j in 0..pivot_idx {
        if arr[j] <= arr[pivot_idx] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_idx);
    i
}

/// Algorithm comparison data
pub struct SortingAlgorithm {
    pub name: &'static str,
    pub best_time: &'static str,
    pub average_time: &'static str,
    pub worst_time: &'static str,
    pub space: &'static str,
    pub stable: bool,
}

pub const ALGORITHMS: &[SortingAlgorithm] = &[
    SortingAlgorithm { name: "Bubble Sort", best_time: "O(n)", average_time: "O(n²)", worst_time: "O(n²)", space: "O(1)", stable: true },
    SortingAlgorithm { name: "Insertion Sort", best_time: "O(n)", average_time: "O(n²)", worst_time: "O(n²)", space: "O(1)", stable: true },
    SortingAlgorithm { name: "Selection Sort", best_time: "O(n²)", average_time: "O(n²)", worst_time: "O(n²)", space: "O(1)", stable: false },
    SortingAlgorithm { name: "Merge Sort", best_time: "O(n log n)", average_time: "O(n log n)", worst_time: "O(n log n)", space: "O(n)", stable: true },
    SortingAlgorithm { name: "Quick Sort", best_time: "O(n log n)", average_time: "O(n log n)", worst_time: "O(n²)", space: "O(log n)", stable: false },
    SortingAlgorithm { name: "Heap Sort", best_time: "O(n log n)", average_time: "O(n log n)", worst_time: "O(n log n)", space: "O(1)", stable: false },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = vec![5, 2, 9, 1, 7, 6, 3];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 6, 7, 9]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut arr = vec![5, 2, 9, 1, 7, 6, 3];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 6, 7, 9]);
    }

    #[test]
    fn test_selection_sort() {
        let mut arr = vec![5, 2, 9, 1, 7, 6, 3];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 6, 7, 9]);
    }

    #[test]
    fn test_merge_sort() {
        let mut arr = vec![5, 2, 9, 1, 7, 6, 3];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 6, 7, 9]);
    }

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![5, 2, 9, 1, 7, 6, 3];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 6, 7, 9]);
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        bubble_sort(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_single() {
        let mut arr = vec![42];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn test_already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }
}

//! # Selection Sort
//!
//! Finds minimum element and places it at the beginning.
//! Time: O(n²), Space: O(1), Stable: No

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

/// Stable selection sort using insertion instead of swap
pub fn stable_selection_sort<T: Ord + Clone>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min_idx = i;
        for j in i + 1..arr.len() {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        let min_val = arr[min_idx].clone();
        for j in (i..min_idx).rev() {
            arr[j + 1] = arr[j].clone();
        }
        arr[i] = min_val;
    }
}

/// Double selection sort (finds min and max simultaneously)
pub fn double_selection_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    let mut left = 0;
    let mut right = n.saturating_sub(1);
    
    while left < right {
        let mut min_idx = left;
        let mut max_idx = left;
        
        for i in left..=right {
            if arr[i] < arr[min_idx] { min_idx = i; }
            if arr[i] > arr[max_idx] { max_idx = i; }
        }
        
        arr.swap(left, min_idx);
        if max_idx == left { max_idx = min_idx; }
        arr.swap(right, max_idx);
        
        left += 1;
        right = right.saturating_sub(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut arr = vec![5, 2, 8, 1, 9];
        selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 5, 8, 9]);
    }

    #[test]
    fn test_stable_selection_sort() {
        let mut arr = vec![5, 2, 8, 1, 9];
        stable_selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 5, 8, 9]);
    }

    #[test]
    fn test_double_selection_sort() {
        let mut arr = vec![5, 2, 8, 1, 9, 3, 7];
        double_selection_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 7, 8, 9]);
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        selection_sort(&mut arr);
        assert!(arr.is_empty());
    }
}

//! # Bubble Sort
//!
//! Simple comparison sort that repeatedly swaps adjacent elements.
//! Time: O(n²), Space: O(1), Stable: Yes

/// Standard bubble sort
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

/// Optimized bubble sort with early termination
pub fn bubble_sort_optimized<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped { break; }
    }
}

/// Cocktail shaker sort (bidirectional bubble sort)
pub fn cocktail_sort<T: Ord>(arr: &mut [T]) {
    let mut start = 0;
    let mut end = arr.len();
    let mut swapped = true;
    
    while swapped {
        swapped = false;
        
        // Forward pass
        for i in start..end - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
        
        if !swapped { break; }
        swapped = false;
        end -= 1;
        
        // Backward pass
        for i in (start..end - 1).rev() {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
        start += 1;
    }
}

/// Count number of swaps (for analysis)
pub fn bubble_sort_count_swaps<T: Ord>(arr: &mut [T]) -> usize {
    let n = arr.len();
    let mut swaps = 0;
    for i in 0..n {
        for j in 0..n - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swaps += 1;
            }
        }
    }
    swaps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = vec![5, 2, 8, 1, 9];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 5, 8, 9]);
    }

    #[test]
    fn test_bubble_sort_optimized() {
        let mut arr = vec![5, 2, 8, 1, 9];
        bubble_sort_optimized(&mut arr);
        assert_eq!(arr, vec![1, 2, 5, 8, 9]);
    }

    #[test]
    fn test_optimized_early_exit() {
        let mut arr = vec![1, 2, 3, 4, 5];
        bubble_sort_optimized(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_cocktail_sort() {
        let mut arr = vec![5, 2, 8, 1, 9];
        cocktail_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 5, 8, 9]);
    }

    #[test]
    fn test_count_swaps() {
        let mut arr = vec![3, 2, 1];
        let swaps = bubble_sort_count_swaps(&mut arr);
        assert_eq!(swaps, 3);
        assert_eq!(arr, vec![1, 2, 3]);
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
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }
}

//! # Insertion Sort
//!
//! Builds sorted array one element at a time.
//! Time: O(n²), Space: O(1), Stable: Yes
//! Excellent for small or nearly sorted arrays.

/// Standard insertion sort
pub fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Binary insertion sort (fewer comparisons, same swaps)
pub fn binary_insertion_sort<T: Ord + Clone>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let key = arr[i].clone();
        let pos = arr[..i].partition_point(|x| x <= &key);
        
        for j in (pos..i).rev() {
            arr[j + 1] = arr[j].clone();
        }
        arr[pos] = key;
    }
}

/// Shell sort (generalized insertion sort)
pub fn shell_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    let mut gap = n / 2;
    
    while gap > 0 {
        for i in gap..n {
            let mut j = i;
            while j >= gap && arr[j - gap] > arr[j] {
                arr.swap(j - gap, j);
                j -= gap;
            }
        }
        gap /= 2;
    }
}

/// Count inversions using insertion sort
pub fn count_inversions<T: Ord>(arr: &mut [T]) -> usize {
    let mut inversions = 0;
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            inversions += 1;
            j -= 1;
        }
    }
    inversions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut arr = vec![5, 2, 8, 1, 9];
        insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 5, 8, 9]);
    }

    #[test]
    fn test_binary_insertion_sort() {
        let mut arr = vec![5, 2, 8, 1, 9];
        binary_insertion_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 5, 8, 9]);
    }

    #[test]
    fn test_shell_sort() {
        let mut arr = vec![5, 2, 8, 1, 9, 3, 7, 4, 6];
        shell_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_nearly_sorted() {
        let mut arr = vec![1, 2, 4, 3, 5];
        let inversions = count_inversions(&mut arr);
        assert_eq!(inversions, 1);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        let inversions = count_inversions(&mut arr);
        assert_eq!(inversions, 10);
    }
}

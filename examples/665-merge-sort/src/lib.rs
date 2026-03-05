//! # Merge Sort
//!
//! Divide and conquer: split, sort halves, merge.
//! Time: O(n log n), Space: O(n), Stable: Yes

/// Top-down merge sort
pub fn merge_sort<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 { return; }
    
    let mid = arr.len() / 2;
    merge_sort(&mut arr[..mid]);
    merge_sort(&mut arr[mid..]);
    
    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();
    merge_into(arr, &left, &right);
}

fn merge_into<T: Ord + Clone>(arr: &mut [T], left: &[T], right: &[T]) {
    let (mut i, mut j, mut k) = (0, 0, 0);
    
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
    
    while i < left.len() { arr[k] = left[i].clone(); i += 1; k += 1; }
    while j < right.len() { arr[k] = right[j].clone(); j += 1; k += 1; }
}

/// Functional merge sort (returns new Vec)
pub fn merge_sort_fn<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    if arr.len() <= 1 { return arr.to_vec(); }
    
    let mid = arr.len() / 2;
    let left = merge_sort_fn(&arr[..mid]);
    let right = merge_sort_fn(&arr[mid..]);
    merge_fn(&left, &right)
}

fn merge_fn<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i].clone());
            i += 1;
        } else {
            result.push(right[j].clone());
            j += 1;
        }
    }
    
    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

/// Bottom-up merge sort (iterative)
pub fn merge_sort_bottom_up<T: Ord + Clone>(arr: &mut [T]) {
    let n = arr.len();
    let mut width = 1;
    
    while width < n {
        let mut i = 0;
        while i < n {
            let mid = (i + width).min(n);
            let end = (i + 2 * width).min(n);
            
            let left = arr[i..mid].to_vec();
            let right = arr[mid..end].to_vec();
            merge_into(&mut arr[i..end], &left, &right);
            
            i += 2 * width;
        }
        width *= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut arr = vec![5, 2, 8, 1, 9, 3];
        merge_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn test_merge_sort_fn() {
        let arr = vec![5, 2, 8, 1, 9, 3];
        let sorted = merge_sort_fn(&arr);
        assert_eq!(sorted, vec![1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn test_bottom_up() {
        let mut arr = vec![5, 2, 8, 1, 9, 3];
        merge_sort_bottom_up(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn test_stable() {
        let mut arr = vec![(3, 'a'), (1, 'b'), (3, 'c'), (1, 'd')];
        merge_sort(&mut arr);
        assert_eq!(arr[0], (1, 'b'));
        assert_eq!(arr[1], (1, 'd'));
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        merge_sort(&mut arr);
        assert!(arr.is_empty());
    }
}

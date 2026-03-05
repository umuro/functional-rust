//! # Quick Sort
//!
//! Divide and conquer with partitioning.
//! Time: O(n log n) average, O(n²) worst, Space: O(log n)

pub fn quick_sort<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 { return; }
    let pivot = partition(arr);
    quick_sort(&mut arr[..pivot]);
    quick_sort(&mut arr[pivot + 1..]);
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot = len - 1;
    let mut i = 0;
    for j in 0..pivot {
        if arr[j] <= arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot);
    i
}

/// Quick sort with median-of-three pivot
pub fn quick_sort_mo3<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 { return; }
    
    let n = arr.len();
    let mid = n / 2;
    
    if arr[0] > arr[mid] { arr.swap(0, mid); }
    if arr[0] > arr[n-1] { arr.swap(0, n-1); }
    if arr[mid] > arr[n-1] { arr.swap(mid, n-1); }
    arr.swap(mid, n-1);
    
    let pivot = partition(arr);
    quick_sort_mo3(&mut arr[..pivot]);
    if pivot + 1 < n { quick_sort_mo3(&mut arr[pivot + 1..]); }
}

/// Three-way partition (Dutch national flag)
pub fn quick_sort_3way<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 { return; }
    
    let pivot = arr[arr.len() - 1].clone();
    let mut lt = 0;
    let mut gt = arr.len() - 1;
    let mut i = 0;
    
    while i <= gt {
        if arr[i] < pivot {
            arr.swap(lt, i);
            lt += 1;
            i += 1;
        } else if arr[i] > pivot {
            arr.swap(i, gt);
            gt -= 1;
        } else {
            i += 1;
        }
    }
    
    quick_sort_3way(&mut arr[..lt]);
    if gt + 1 < arr.len() { quick_sort_3way(&mut arr[gt + 1..]); }
}

/// Functional quick sort (returns new Vec)
pub fn quick_sort_fn<T: Ord + Clone>(arr: &[T]) -> Vec<T> {
    if arr.len() <= 1 { return arr.to_vec(); }
    
    let pivot = &arr[0];
    let left: Vec<_> = arr[1..].iter().filter(|x| *x < pivot).cloned().collect();
    let right: Vec<_> = arr[1..].iter().filter(|x| *x >= pivot).cloned().collect();
    
    let mut result = quick_sort_fn(&left);
    result.push(pivot.clone());
    result.extend(quick_sort_fn(&right));
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![5, 2, 8, 1, 9, 3];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn test_quick_sort_mo3() {
        let mut arr = vec![5, 2, 8, 1, 9, 3, 7, 4, 6];
        quick_sort_mo3(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_quick_sort_3way() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        quick_sort_3way(&mut arr);
        assert_eq!(arr, vec![1, 1, 2, 3, 3, 4, 5, 5, 5, 6, 9]);
    }

    #[test]
    fn test_quick_sort_fn() {
        let arr = vec![5, 2, 8, 1, 9, 3];
        let sorted = quick_sort_fn(&arr);
        assert_eq!(sorted, vec![1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn test_duplicates() {
        let mut arr = vec![5, 5, 5, 5];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![5, 5, 5, 5]);
    }
}

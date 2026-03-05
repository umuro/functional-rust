//! # Heap Sort
//!
//! Uses binary heap for guaranteed O(n log n) with O(1) space.

pub fn heap_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    
    // Build max heap
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }
    
    // Extract elements
    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    
    if left < n && arr[left] > arr[largest] { largest = left; }
    if right < n && arr[right] > arr[largest] { largest = right; }
    
    if largest != i {
        arr.swap(i, largest);
        heapify(arr, n, largest);
    }
}

/// Iterative heapify (avoids stack overflow for large arrays)
pub fn heap_sort_iterative<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    
    for i in (0..n / 2).rev() {
        sift_down(arr, n, i);
    }
    
    for i in (1..n).rev() {
        arr.swap(0, i);
        sift_down(arr, i, 0);
    }
}

fn sift_down<T: Ord>(arr: &mut [T], n: usize, mut i: usize) {
    loop {
        let mut largest = i;
        let left = 2 * i + 1;
        let right = 2 * i + 2;
        
        if left < n && arr[left] > arr[largest] { largest = left; }
        if right < n && arr[right] > arr[largest] { largest = right; }
        
        if largest == i { break; }
        arr.swap(i, largest);
        i = largest;
    }
}

/// Find k largest elements
pub fn k_largest<T: Ord + Clone>(arr: &[T], k: usize) -> Vec<T> {
    let mut heap = arr.to_vec();
    let n = heap.len();
    
    for i in (0..n / 2).rev() {
        heapify(&mut heap, n, i);
    }
    
    let mut result = Vec::with_capacity(k);
    for _ in 0..k.min(n) {
        result.push(heap[0].clone());
        heap.swap(0, heap.len() - 1);
        heap.pop();
        if !heap.is_empty() {
            heapify(&mut heap, heap.len(), 0);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_sort() {
        let mut arr = vec![5, 2, 8, 1, 9, 3];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn test_heap_sort_iterative() {
        let mut arr = vec![5, 2, 8, 1, 9, 3];
        heap_sort_iterative(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 5, 8, 9]);
    }

    #[test]
    fn test_k_largest() {
        let arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let top3 = k_largest(&arr, 3);
        assert_eq!(top3, vec![9, 6, 5]);
    }

    #[test]
    fn test_empty() {
        let mut arr: Vec<i32> = vec![];
        heap_sort(&mut arr);
        assert!(arr.is_empty());
    }

    #[test]
    fn test_single() {
        let mut arr = vec![42];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![42]);
    }
}

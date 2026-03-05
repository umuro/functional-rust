//! # Introsort
//!
//! Hybrid: quicksort + heapsort + insertion sort.
//! Switches to heapsort when recursion depth exceeds 2*log(n).

pub fn introsort<T: Ord>(arr: &mut [T]) {
    let max_depth = 2 * (arr.len() as f64).log2() as usize;
    introsort_impl(arr, max_depth);
}

fn introsort_impl<T: Ord>(arr: &mut [T], depth_limit: usize) {
    if arr.len() <= 16 {
        insertion_sort(arr);
    } else if depth_limit == 0 {
        heap_sort(arr);
    } else {
        let p = partition(arr);
        introsort_impl(&mut arr[..p], depth_limit - 1);
        introsort_impl(&mut arr[p + 1..], depth_limit - 1);
    }
}

fn insertion_sort<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] { arr.swap(j - 1, j); j -= 1; }
    }
}

fn partition<T: Ord>(arr: &mut [T]) -> usize {
    let n = arr.len();
    let mut i = 0;
    for j in 0..n - 1 { if arr[j] <= arr[n - 1] { arr.swap(i, j); i += 1; } }
    arr.swap(i, n - 1);
    i
}

fn heap_sort<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in (0..n / 2).rev() { heapify(arr, n, i); }
    for i in (1..n).rev() { arr.swap(0, i); heapify(arr, i, 0); }
}

fn heapify<T: Ord>(arr: &mut [T], n: usize, i: usize) {
    let (mut largest, left, right) = (i, 2 * i + 1, 2 * i + 2);
    if left < n && arr[left] > arr[largest] { largest = left; }
    if right < n && arr[right] > arr[largest] { largest = right; }
    if largest != i { arr.swap(i, largest); heapify(arr, n, largest); }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_introsort() {
        let mut arr = vec![5, 2, 8, 1, 9, 3, 7, 4, 6];
        introsort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}

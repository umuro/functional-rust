//! # BinaryHeap Priority Queue
//! Max-heap for priority queue operations.

use std::collections::BinaryHeap;
use std::cmp::Reverse;

pub fn top_n<T: Ord + Clone>(items: &[T], n: usize) -> Vec<T> {
    let mut heap: BinaryHeap<_> = items.iter().cloned().collect();
    (0..n).filter_map(|_| heap.pop()).collect()
}

pub fn bottom_n<T: Ord + Clone>(items: &[T], n: usize) -> Vec<T> {
    let mut heap: BinaryHeap<Reverse<T>> = items.iter().cloned().map(Reverse).collect();
    (0..n).filter_map(|_| heap.pop().map(|Reverse(x)| x)).collect()
}

pub fn heap_sort<T: Ord>(items: Vec<T>) -> Vec<T> {
    let heap: BinaryHeap<_> = items.into_iter().collect();
    heap.into_sorted_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn top_3() {
        let top = top_n(&[3, 1, 4, 1, 5, 9, 2, 6], 3);
        assert_eq!(top, vec![9, 6, 5]);
    }
    #[test] fn bottom_3() {
        let bottom = bottom_n(&[3, 1, 4, 1, 5, 9, 2, 6], 3);
        assert_eq!(bottom, vec![1, 1, 2]);
    }
    #[test] fn sorted() {
        assert_eq!(heap_sort(vec![3, 1, 4, 1, 5]), vec![1, 1, 3, 4, 5]);
    }
}

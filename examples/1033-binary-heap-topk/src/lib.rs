#![allow(dead_code)]
#![allow(clippy::all)]
// 1033: Top-K Elements with BinaryHeap
// BinaryHeap is a max-heap; use Reverse<T> for min-heap behavior

use std::cmp::Reverse;
use std::collections::BinaryHeap;

/// Top-K using a min-heap of size K
/// Keep only the K largest elements by evicting the smallest
fn top_k(k: usize, data: &[i32]) -> Vec<i32> {
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::with_capacity(k + 1);

    for &val in data {
        heap.push(Reverse(val));
        if heap.len() > k {
            heap.pop(); // Remove smallest (which is the max in Reverse heap)
        }
    }

    let mut result: Vec<i32> = heap.into_iter().map(|Reverse(x)| x).collect();
    result.sort_unstable_by(|a, b| b.cmp(a)); // Descending
    result
}

fn test_top_k() {
    let data = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    let result = top_k(3, &data);
    assert_eq!(result, vec![9, 6, 5]);

    let result = top_k(1, &data);
    assert_eq!(result, vec![9]);
}

/// BinaryHeap as a max-heap: natural ordering
fn max_heap_demo() {
    let mut heap = BinaryHeap::new();
    heap.push(3);
    heap.push(1);
    heap.push(4);
    heap.push(1);
    heap.push(5);

    // pop() returns largest first
    assert_eq!(heap.pop(), Some(5));
    assert_eq!(heap.pop(), Some(4));
    assert_eq!(heap.pop(), Some(3));

    // peek without removing
    assert_eq!(heap.peek(), Some(&1));
}

/// Top-K with custom key function
fn top_k_by<T, K, F>(k: usize, data: &[T], key_fn: F) -> Vec<&T>
where
    K: Ord,
    F: Fn(&T) -> K,
{
    let mut heap: BinaryHeap<Reverse<(K, usize)>> = BinaryHeap::new();

    for (i, item) in data.iter().enumerate() {
        heap.push(Reverse((key_fn(item), i)));
        if heap.len() > k {
            heap.pop();
        }
    }

    let mut indices: Vec<usize> = heap.into_iter().map(|Reverse((_, i))| i).collect();
    indices.sort_by(|&a, &b| key_fn(&data[b]).cmp(&key_fn(&data[a])).then(b.cmp(&a)));
    indices.iter().map(|&i| &data[i]).collect()
}

fn test_top_k_by() {
    let words = vec!["hi", "hello", "hey", "howdy", "h"];
    let longest3 = top_k_by(3, &words, |w| w.len());
    assert_eq!(longest3.len(), 3);
    assert_eq!(*longest3[0], "howdy");
    assert_eq!(*longest3[1], "hello");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_top_k_basic() {
        test_top_k();
    }

    #[test]
    fn test_max_heap() {
        max_heap_demo();
    }

    #[test]
    fn test_custom_key() {
        test_top_k_by();
    }

    #[test]
    fn test_from_vec() {
        let v = vec![5, 3, 8, 1, 9];
        let heap: BinaryHeap<_> = v.into_iter().collect();
        let sorted: Vec<_> = heap.into_sorted_vec();
        assert_eq!(sorted, vec![1, 3, 5, 8, 9]); // ascending
    }

    #[test]
    fn test_min_heap() {
        let mut min_heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
        min_heap.push(Reverse(5));
        min_heap.push(Reverse(1));
        min_heap.push(Reverse(3));
        assert_eq!(min_heap.pop(), Some(Reverse(1))); // smallest first
    }
}

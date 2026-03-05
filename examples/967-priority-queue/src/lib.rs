// 967: Priority Queue
// Approach 1: Manual min-heap implementation
// Approach 2: std::collections::BinaryHeap (max-heap, wrap for min)

// --- Approach 1: Manual min-heap ---
pub struct MinHeap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        MinHeap { data: Vec::new() }
    }

    pub fn push(&mut self, x: T) {
        self.data.push(x);
        self.sift_up(self.data.len() - 1);
    }

    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let last = self.data.len() - 1;
        self.data.swap(0, last);
        let top = self.data.pop();
        if !self.data.is_empty() {
            self.sift_down(0);
        }
        top
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    fn sift_up(&mut self, mut i: usize) {
        while i > 0 {
            let parent = (i - 1) / 2;
            if self.data[i] < self.data[parent] {
                self.data.swap(i, parent);
                i = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut i: usize) {
        loop {
            let left = 2 * i + 1;
            let right = 2 * i + 2;
            let mut smallest = i;
            if left < self.data.len() && self.data[left] < self.data[smallest] {
                smallest = left;
            }
            if right < self.data.len() && self.data[right] < self.data[smallest] {
                smallest = right;
            }
            if smallest != i {
                self.data.swap(i, smallest);
                i = smallest;
            } else {
                break;
            }
        }
    }
}

impl<T: Ord> Default for MinHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

// --- Approach 2: std BinaryHeap (max-heap; use Reverse for min-heap) ---
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn heap_sort(mut values: Vec<i32>) -> Vec<i32> {
    let mut heap: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    for v in values.drain(..) {
        heap.push(Reverse(v));
    }
    let mut sorted = Vec::with_capacity(heap.len());
    while let Some(Reverse(v)) = heap.pop() {
        sorted.push(v);
    }
    sorted
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_heap_order() {
        let mut h: MinHeap<i32> = MinHeap::new();
        h.push(5);
        h.push(3);
        h.push(8);
        h.push(1);
        h.push(9);
        h.push(2);

        assert_eq!(h.peek(), Some(&1));
        assert_eq!(h.size(), 6);
        assert_eq!(h.pop(), Some(1));
        assert_eq!(h.pop(), Some(2));
        assert_eq!(h.pop(), Some(3));
        assert_eq!(h.pop(), Some(5));
        assert_eq!(h.pop(), Some(8));
        assert_eq!(h.pop(), Some(9));
        assert_eq!(h.pop(), None);
    }

    #[test]
    fn test_heap_sort_manual() {
        let mut h: MinHeap<i32> = MinHeap::new();
        for v in [4, 7, 2, 1, 8, 3, 6, 5] {
            h.push(v);
        }
        let mut sorted = vec![];
        while let Some(v) = h.pop() {
            sorted.push(v);
        }
        assert_eq!(sorted, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_std_binary_heap_min() {
        let sorted = heap_sort(vec![4, 7, 2, 1, 8, 3, 6, 5]);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_empty() {
        let mut h: MinHeap<i32> = MinHeap::new();
        assert!(h.is_empty());
        assert_eq!(h.peek(), None);
        assert_eq!(h.pop(), None);
    }

    #[test]
    fn test_single() {
        let mut h: MinHeap<i32> = MinHeap::new();
        h.push(42);
        assert_eq!(h.peek(), Some(&42));
        assert_eq!(h.size(), 1);
        assert_eq!(h.pop(), Some(42));
        assert!(h.is_empty());
    }
}

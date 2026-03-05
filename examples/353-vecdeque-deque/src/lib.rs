//! # VecDeque
//! Double-ended queue with O(1) push/pop on both ends.

use std::collections::VecDeque;

pub fn sliding_window(data: &[i32], window_size: usize) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut window: VecDeque<i32> = VecDeque::new();
    for &item in data {
        window.push_back(item);
        if window.len() > window_size { window.pop_front(); }
        if window.len() == window_size {
            result.push(window.iter().cloned().collect());
        }
    }
    result
}

pub fn rotate_left<T: Clone>(items: &[T], n: usize) -> Vec<T> {
    let mut dq: VecDeque<_> = items.iter().cloned().collect();
    dq.rotate_left(n % items.len().max(1));
    dq.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn sliding_windows() {
        let windows = sliding_window(&[1, 2, 3, 4, 5], 3);
        assert_eq!(windows, vec![vec![1,2,3], vec![2,3,4], vec![3,4,5]]);
    }
    #[test] fn rotation() {
        assert_eq!(rotate_left(&[1, 2, 3, 4, 5], 2), vec![3, 4, 5, 1, 2]);
    }
    #[test] fn deque_both_ends() {
        let mut dq = VecDeque::new();
        dq.push_back(2); dq.push_front(1); dq.push_back(3);
        assert_eq!(dq.pop_front(), Some(1));
        assert_eq!(dq.pop_back(), Some(3));
    }
}

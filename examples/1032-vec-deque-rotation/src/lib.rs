// 1032: VecDeque Rotation — Efficient Front/Back Operations
// VecDeque is a ring buffer: O(1) push/pop at both ends

use std::collections::VecDeque;

/// Basic front/back operations
fn basic_deque() {
    let mut dq = VecDeque::new();
    dq.push_back(1);
    dq.push_back(2);
    dq.push_back(3);
    dq.push_front(0);

    assert_eq!(dq.pop_front(), Some(0));
    assert_eq!(dq.pop_front(), Some(1));
    assert_eq!(dq.pop_back(), Some(3));
    assert_eq!(dq.pop_back(), Some(2));
    assert!(dq.is_empty());
}

/// Rotation using VecDeque::rotate_left/rotate_right
fn rotation() {
    let mut dq: VecDeque<_> = [1, 2, 3, 4, 5].into_iter().collect();

    // Rotate left by 2: [3, 4, 5, 1, 2]
    dq.rotate_left(2);
    assert_eq!(dq, [3, 4, 5, 1, 2].into_iter().collect::<VecDeque<_>>());

    // Rotate right by 2: back to [1, 2, 3, 4, 5]
    dq.rotate_right(2);
    assert_eq!(dq, [1, 2, 3, 4, 5].into_iter().collect::<VecDeque<_>>());
}

/// Using VecDeque as a sliding window
fn sliding_window() {
    let data = vec![1, 2, 3, 4, 5, 6, 7];
    let window_size = 3;
    let mut window: VecDeque<i32> = VecDeque::with_capacity(window_size);
    let mut sums = Vec::new();

    for &val in &data {
        window.push_back(val);
        if window.len() > window_size {
            window.pop_front();
        }
        if window.len() == window_size {
            let sum: i32 = window.iter().sum();
            sums.push(sum);
        }
    }

    assert_eq!(sums, vec![6, 9, 12, 15, 18]); // 1+2+3, 2+3+4, ...
}

/// VecDeque to Vec and back
fn conversions() {
    let v = vec![1, 2, 3, 4, 5];
    let mut dq: VecDeque<_> = v.into_iter().collect();

    dq.push_front(0);
    // make_contiguous ensures internal buffer is contiguous
    dq.make_contiguous();

    let v: Vec<_> = dq.into_iter().collect();
    assert_eq!(v, vec![0, 1, 2, 3, 4, 5]);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_deque() { basic_deque(); }

    #[test]
    fn test_rotation() { rotation(); }

    #[test]
    fn test_sliding_window() { sliding_window(); }

    #[test]
    fn test_conversions() { conversions(); }

    #[test]
    fn test_indexed_access() {
        let dq: VecDeque<_> = [10, 20, 30].into_iter().collect();
        assert_eq!(dq[0], 10);
        assert_eq!(dq[1], 20);
        assert_eq!(dq[2], 30);
    }
}

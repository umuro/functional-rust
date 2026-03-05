// 1040: Queue Using VecDeque
// FIFO queue: push_back, pop_front — both O(1)

use std::collections::VecDeque;

/// Queue wrapper (optional — VecDeque is already a queue)
struct Queue<T> {
    inner: VecDeque<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Queue { inner: VecDeque::new() }
    }

    fn enqueue(&mut self, value: T) {
        self.inner.push_back(value);
    }

    fn dequeue(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    fn peek(&self) -> Option<&T> {
        self.inner.front()
    }

    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    fn len(&self) -> usize {
        self.inner.len()
    }
}

fn basic_queue() {
    let mut q = Queue::new();
    q.enqueue(1);
    q.enqueue(2);
    q.enqueue(3);

    assert_eq!(q.len(), 3);
    assert_eq!(q.peek(), Some(&1));
    assert_eq!(q.dequeue(), Some(1));
    assert_eq!(q.dequeue(), Some(2));
    assert_eq!(q.dequeue(), Some(3));
    assert!(q.is_empty());
}

/// VecDeque directly as a queue
fn vecdeque_as_queue() {
    let mut q: VecDeque<&str> = VecDeque::new();
    q.push_back("first");
    q.push_back("second");
    q.push_back("third");

    assert_eq!(q.front(), Some(&"first"));
    assert_eq!(q.pop_front(), Some("first"));
    assert_eq!(q.pop_front(), Some("second"));
    assert_eq!(q.len(), 1);
}

/// BFS with level tracking using queue
fn bfs_levels(adjacency: &[Vec<usize>], start: usize) -> Vec<Vec<usize>> {
    let n = adjacency.len();
    let mut visited = vec![false; n];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut levels: Vec<Vec<usize>> = Vec::new();

    visited[start] = true;
    queue.push_back((start, 0));

    while let Some((node, level)) = queue.pop_front() {
        // Extend levels vec if needed
        while levels.len() <= level {
            levels.push(Vec::new());
        }
        levels[level].push(node);

        for &neighbor in &adjacency[node] {
            if !visited[neighbor] {
                visited[neighbor] = true;
                queue.push_back((neighbor, level + 1));
            }
        }
    }
    levels
}

fn bfs_test() {
    let adj = vec![
        vec![1, 2], // 0 -> 1, 2
        vec![3],    // 1 -> 3
        vec![3],    // 2 -> 3
        vec![],     // 3 -> (none)
    ];

    let levels = bfs_levels(&adj, 0);
    assert_eq!(levels[0], vec![0]);
    assert_eq!(levels[1], vec![1, 2]);
    assert_eq!(levels[2], vec![3]);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() { basic_queue(); }

    #[test]
    fn test_vecdeque() { vecdeque_as_queue(); }

    #[test]
    fn test_bfs() { bfs_test(); }

    #[test]
    fn test_empty_dequeue() {
        let mut q: Queue<i32> = Queue::new();
        assert_eq!(q.dequeue(), None);
        assert_eq!(q.peek(), None);
    }
}

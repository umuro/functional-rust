// Example 096: ExactSizeIterator
// When you know the length upfront — O(1) .len(), exact pre-allocation, progress tracking.

// === Approach 1: Idiomatic — using ExactSizeIterator::len() on slices ===

/// Process items while reporting progress using the exact known length.
/// `.iter()` on a slice implements ExactSizeIterator, so `.len()` is O(1).
pub fn process_with_progress(data: &[i32]) -> Vec<String> {
    let total = data.len(); // O(1) via ExactSizeIterator
    data.iter()
        .enumerate()
        .map(|(i, &x)| format!("[{}/{}] Processing {}", i + 1, total, x))
        .collect()
}

/// Render an ASCII progress bar given a completed count, total, and bar width.
pub fn progress_bar(completed: usize, total: usize, width: usize) -> String {
    if total == 0 {
        return format!("[{}] 0/0", ".".repeat(width));
    }
    let filled = completed * width / total;
    let empty = width - filled;
    format!(
        "[{}{}] {}/{}",
        "#".repeat(filled),
        ".".repeat(empty),
        completed,
        total
    )
}

// === Approach 2: Pre-allocate with exact capacity from ExactSizeIterator ===

/// Map a function over a slice, pre-allocating the output Vec exactly.
/// Because `data.len()` is O(1), `Vec::with_capacity` avoids all reallocations.
pub fn map_preallocated<T, U>(data: &[T], f: impl Fn(&T) -> U) -> Vec<U> {
    let mut result = Vec::with_capacity(data.len());
    for item in data {
        result.push(f(item));
    }
    result
}

/// Split a slice into chunks of exactly `n` elements, discarding any remainder.
/// Returns a Vec of owned Vec chunks.
pub fn chunks_exact(data: &[i32], n: usize) -> Vec<Vec<i32>> {
    if n == 0 {
        return vec![];
    }
    data.chunks_exact(n).map(|c| c.to_vec()).collect()
}

// === Approach 3: Custom ExactSizeIterator ===

/// A counter iterator that counts down from `remaining` to 0.
/// Implements ExactSizeIterator so callers can query `.len()` in O(1).
pub struct Countdown {
    remaining: usize,
}

impl Countdown {
    pub fn new(n: usize) -> Self {
        Countdown { remaining: n }
    }
}

impl Iterator for Countdown {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.remaining == 0 {
            None
        } else {
            self.remaining -= 1;
            Some(self.remaining)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

// Implementing ExactSizeIterator requires size_hint to be exact.
impl ExactSizeIterator for Countdown {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_with_progress_empty() {
        let result = process_with_progress(&[]);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_process_with_progress_single() {
        let result = process_with_progress(&[42]);
        assert_eq!(result, vec!["[1/1] Processing 42"]);
    }

    #[test]
    fn test_process_with_progress_multiple() {
        let result = process_with_progress(&[10, 20, 30]);
        assert_eq!(
            result,
            vec![
                "[1/3] Processing 10",
                "[2/3] Processing 20",
                "[3/3] Processing 30",
            ]
        );
    }

    #[test]
    fn test_progress_bar_empty_total() {
        let bar = progress_bar(0, 0, 10);
        assert_eq!(bar, "[..........] 0/0");
    }

    #[test]
    fn test_progress_bar_half() {
        let bar = progress_bar(5, 10, 10);
        assert_eq!(bar, "[#####.....] 5/10");
    }

    #[test]
    fn test_progress_bar_full() {
        let bar = progress_bar(10, 10, 10);
        assert_eq!(bar, "[##########] 10/10");
    }

    #[test]
    fn test_map_preallocated() {
        let data = vec![1, 2, 3, 4];
        let result = map_preallocated(&data, |&x| x * 2);
        assert_eq!(result, vec![2, 4, 6, 8]);
    }

    #[test]
    fn test_map_preallocated_empty() {
        let result = map_preallocated(&[] as &[i32], |&x| x + 1);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_chunks_exact() {
        let data = vec![1, 2, 3, 4, 5, 6, 7];
        let chunks = chunks_exact(&data, 3);
        assert_eq!(chunks, vec![vec![1, 2, 3], vec![4, 5, 6]]);
        // 7 is the remainder — discarded by chunks_exact
    }

    #[test]
    fn test_chunks_exact_zero_n() {
        let chunks = chunks_exact(&[1, 2, 3], 0);
        assert_eq!(chunks, Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test_countdown_len() {
        let mut cd = Countdown::new(5);
        assert_eq!(cd.len(), 5);
        cd.next();
        assert_eq!(cd.len(), 4);
        cd.next();
        cd.next();
        assert_eq!(cd.len(), 2);
    }

    #[test]
    fn test_countdown_values() {
        let values: Vec<usize> = Countdown::new(3).collect();
        assert_eq!(values, vec![2, 1, 0]);
    }

    #[test]
    fn test_countdown_empty() {
        let values: Vec<usize> = Countdown::new(0).collect();
        assert_eq!(values, Vec::<usize>::new());
    }

    #[test]
    fn test_exact_size_enables_preallocation() {
        // Demonstrate that ExactSizeIterator adapters preserve size information:
        // .map() on a slice iterator is also ExactSizeIterator
        let data = [1i32, 2, 3, 4, 5];
        let mapped = data.iter().map(|&x| x * 2);
        assert_eq!(mapped.len(), 5); // preserved through .map()

        // .enumerate() also preserves it
        let enumerated = data.iter().enumerate();
        assert_eq!(enumerated.len(), 5);
    }
}

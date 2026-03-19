#![allow(clippy::all)]
//! Iterator Trait Deep Dive
//!
//! Advanced iterator patterns, adapters, and custom iterators.

/// Custom Fibonacci iterator.
pub struct Fibonacci {
    curr: u64,
    next: u64,
}

impl Fibonacci {
    /// Creates a new Fibonacci iterator starting with 1, 1, 2, 3, ...
    pub fn new() -> Self {
        Fibonacci { curr: 0, next: 1 }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.curr + self.next;
        self.curr = self.next;
        self.next = new_next;
        Some(self.curr)
    }
}

/// Custom iterator that counts down from n to 1.
pub struct Countdown {
    current: u32,
}

impl Countdown {
    pub fn new(start: u32) -> Self {
        Countdown { current: start }
    }
}

impl Iterator for Countdown {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > 0 {
            let value = self.current;
            self.current -= 1;
            Some(value)
        } else {
            None
        }
    }

    // Provide size_hint for efficiency
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.current as usize;
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for Countdown {}

/// Approach 1: Using standard combinators for sum of squares of evens.
pub fn sum_of_squares_of_evens(n: i32) -> i32 {
    (1..=n).filter(|x| x % 2 == 0).map(|x| x * x).sum()
}

/// Approach 2: Using fold for more control.
pub fn sum_of_squares_of_evens_fold(n: i32) -> i32 {
    (1..=n).fold(0, |acc, x| if x % 2 == 0 { acc + x * x } else { acc })
}

/// Demonstrates flat_map for Cartesian product.
pub fn pairs_where_x_less_than_y(n: i32) -> Vec<(i32, i32)> {
    (1..=n)
        .flat_map(|x| (1..=n).map(move |y| (x, y)))
        .filter(|(x, y)| x < y)
        .collect()
}

/// Demonstrates scan for running totals.
pub fn running_sum(values: &[i32]) -> Vec<i32> {
    values
        .iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect()
}

/// Demonstrates take_while.
pub fn take_while_positive(values: &[i32]) -> Vec<i32> {
    values.iter().take_while(|&&x| x > 0).copied().collect()
}

/// Demonstrates skip_while.
pub fn skip_while_small(values: &[i32], threshold: i32) -> Vec<i32> {
    values
        .iter()
        .skip_while(|&&x| x < threshold)
        .copied()
        .collect()
}

/// Demonstrates chain for concatenation.
pub fn chain_ranges(
    a: std::ops::RangeInclusive<i32>,
    b: std::ops::RangeInclusive<i32>,
) -> Vec<i32> {
    a.chain(b).collect()
}

/// Demonstrates partition.
pub fn partition_even_odd(n: i32) -> (Vec<i32>, Vec<i32>) {
    (1..=n).partition(|x| x % 2 == 0)
}

/// Demonstrates zip.
pub fn zip_with_index<T: Clone>(items: &[T]) -> Vec<(usize, T)> {
    (0..).zip(items.iter().cloned()).collect()
}

/// Demonstrates unzip.
pub fn unzip_pairs<A: Clone, B: Clone>(pairs: Vec<(A, B)>) -> (Vec<A>, Vec<B>) {
    pairs.into_iter().unzip()
}

/// Demonstrates peekable for lookahead.
pub fn group_consecutive(values: &[i32]) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut iter = values.iter().peekable();

    while let Some(&first) = iter.next() {
        let mut group = vec![first];
        while iter.peek() == Some(&&first) {
            iter.next();
            group.push(first);
        }
        result.push(group);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci() {
        let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
        assert_eq!(fibs, vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
    }

    #[test]
    fn test_countdown() {
        let count: Vec<u32> = Countdown::new(5).collect();
        assert_eq!(count, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_countdown_exact_size() {
        let cd = Countdown::new(10);
        assert_eq!(cd.len(), 10);
    }

    #[test]
    fn test_sum_of_squares_of_evens() {
        // 2² + 4² + 6² + 8² + 10² = 4 + 16 + 36 + 64 + 100 = 220
        assert_eq!(sum_of_squares_of_evens(10), 220);
        assert_eq!(sum_of_squares_of_evens_fold(10), 220);
    }

    #[test]
    fn test_pairs_where_x_less_than_y() {
        let pairs = pairs_where_x_less_than_y(3);
        assert_eq!(pairs, vec![(1, 2), (1, 3), (2, 3)]);
    }

    #[test]
    fn test_running_sum() {
        assert_eq!(running_sum(&[1, 2, 3, 4]), vec![1, 3, 6, 10]);
    }

    #[test]
    fn test_take_while_positive() {
        assert_eq!(take_while_positive(&[1, 2, 3, -1, 4, 5]), vec![1, 2, 3]);
    }

    #[test]
    fn test_skip_while_small() {
        assert_eq!(skip_while_small(&[1, 2, 5, 3, 6], 4), vec![5, 3, 6]);
    }

    #[test]
    fn test_chain_ranges() {
        assert_eq!(chain_ranges(1..=3, 7..=9), vec![1, 2, 3, 7, 8, 9]);
    }

    #[test]
    fn test_partition_even_odd() {
        let (evens, odds) = partition_even_odd(6);
        assert_eq!(evens, vec![2, 4, 6]);
        assert_eq!(odds, vec![1, 3, 5]);
    }

    #[test]
    fn test_zip_with_index() {
        let items = vec!["a", "b", "c"];
        let indexed = zip_with_index(&items);
        assert_eq!(indexed, vec![(0, "a"), (1, "b"), (2, "c")]);
    }

    #[test]
    fn test_unzip_pairs() {
        let pairs = vec![("a", 1), ("b", 2), ("c", 3)];
        let (letters, numbers) = unzip_pairs(pairs);
        assert_eq!(letters, vec!["a", "b", "c"]);
        assert_eq!(numbers, vec![1, 2, 3]);
    }

    #[test]
    fn test_group_consecutive() {
        let groups = group_consecutive(&[1, 1, 2, 3, 3, 3, 1]);
        assert_eq!(groups, vec![vec![1, 1], vec![2], vec![3, 3, 3], vec![1]]);
    }
}

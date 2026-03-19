#![allow(clippy::all)]
// Example 255: Lazy Fibonacci — Infinite stream using closures/iterators
//
// OCaml uses a recursive `stream` type with thunks (`unit -> 'a stream`) to
// model infinite lazy sequences.  Rust offers two natural analogues:
//   1. A custom `Stream` struct that boxes the thunk (mirrors OCaml directly)
//   2. `std::iter::Iterator` — the idiomatic Rust lazy-sequence abstraction

// ---------------------------------------------------------------------------
// Solution 1: Idiomatic Rust — Iterator
//
// Rust's Iterator trait is the canonical lazy sequence.
// `FibIter` holds only the two most-recent values; no heap allocation needed.
// ---------------------------------------------------------------------------

/// Infinite Fibonacci iterator starting from (a, b).
pub struct FibIter {
    a: u64,
    b: u64,
}

impl FibIter {
    /// Create a new Fibonacci iterator.
    /// `fibs()` starts at 0, 1, 1, 2, 3, 5, …
    pub fn new(a: u64, b: u64) -> Self {
        Self { a, b }
    }
}

impl Iterator for FibIter {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let value = self.a;
        // Advance: (a, b) → (b, a + b)
        let next_b = self.a + self.b;
        self.a = self.b;
        self.b = next_b;
        Some(value) // infinite — always Some
    }
}

/// Collect the first `n` Fibonacci numbers.
pub fn fibs_take(n: usize) -> Vec<u64> {
    FibIter::new(0, 1).take(n).collect()
}

// ---------------------------------------------------------------------------
// Solution 2: Functional / thunk-based — mirrors the OCaml stream type
//
// OCaml:  type 'a stream = Cons of 'a * (unit -> 'a stream)
//
// We encode this with a `Box<dyn Fn() -> Stream<T>>` thunk.
// The `Box` is necessary because a recursive type must have known size.
// ---------------------------------------------------------------------------

/// A singly-linked lazy stream; each tail is a heap-allocated thunk.
pub struct Stream<T> {
    pub head: T,
    // `Box<dyn Fn()>` gives us a heap-allocated, callable thunk —
    // exactly like OCaml's `unit -> 'a stream`.
    tail: Box<dyn Fn() -> Stream<T>>,
}

impl<T: Copy> Stream<T> {
    /// Collect the first `n` elements into a `Vec`.
    pub fn take(&self, n: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(n);
        if n == 0 {
            return result;
        }
        result.push(self.head);
        let mut next = (self.tail)();
        for _ in 1..n {
            result.push(next.head);
            next = (next.tail)();
        }
        result
    }
}

/// Build an infinite Fibonacci stream starting from `(a, b)`.
///
/// Mirrors the OCaml:  `let rec fibs a b = Cons (a, fun () -> fibs b (a + b))`
pub fn fibs_stream(a: u64, b: u64) -> Stream<u64> {
    Stream {
        head: a,
        tail: Box::new(move || fibs_stream(b, a + b)),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -- Iterator-based tests ------------------------------------------------

    #[test]
    fn test_iter_empty() {
        let result: Vec<u64> = FibIter::new(0, 1).take(0).collect();
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_iter_single() {
        let result: Vec<u64> = FibIter::new(0, 1).take(1).collect();
        assert_eq!(result, vec![0]);
    }

    #[test]
    fn test_iter_ten() {
        assert_eq!(fibs_take(10), vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_iter_large() {
        // fib(0..19): 0,1,1,2,3,5,8,13,21,34,55,89,144,233,377,610,987,1597,2584,4181
        let v = fibs_take(20);
        assert_eq!(v[19], 4181);
    }

    #[test]
    fn test_iter_non_standard_start() {
        // Starting from (1, 1) yields Lucas-adjacent sequence
        let result: Vec<u64> = FibIter::new(1, 1).take(5).collect();
        assert_eq!(result, vec![1, 1, 2, 3, 5]);
    }

    // -- Stream (thunk) tests ------------------------------------------------

    #[test]
    fn test_stream_empty() {
        let s = fibs_stream(0, 1);
        assert_eq!(s.take(0), vec![]);
    }

    #[test]
    fn test_stream_single() {
        let s = fibs_stream(0, 1);
        assert_eq!(s.take(1), vec![0]);
    }

    #[test]
    fn test_stream_ten() {
        let s = fibs_stream(0, 1);
        assert_eq!(s.take(10), vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_stream_matches_iter() {
        let iter_result = fibs_take(15);
        let stream_result = fibs_stream(0, 1).take(15);
        assert_eq!(iter_result, stream_result);
    }
}

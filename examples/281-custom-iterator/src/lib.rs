#![allow(clippy::all)]
//! # Custom Iterator Implementation
//!
//! Demonstrates implementing the `Iterator` trait from scratch.
//! Only `next()` is required — the rest of the iterator API comes for free.

/// A counter that yields squares up to a maximum
pub struct Squares {
    current: u32,
    max: u32,
}

impl Squares {
    pub fn new(max: u32) -> Self {
        Squares { current: 0, max }
    }
}

impl Iterator for Squares {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.max {
            return None;
        }
        let val = self.current * self.current;
        self.current += 1;
        Some(val)
    }
}

/// Fibonacci sequence iterator - infinite, always returns Some
pub struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    pub fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
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
        let val = self.a;
        let next = self.a + self.b;
        self.a = self.b;
        self.b = next;
        Some(val) // infinite — always Some
    }
}

/// Alternative: A range iterator using step
pub struct SteppedRange {
    current: i32,
    end: i32,
    step: i32,
}

impl SteppedRange {
    pub fn new(start: i32, end: i32, step: i32) -> Self {
        SteppedRange {
            current: start,
            end,
            step,
        }
    }
}

impl Iterator for SteppedRange {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if (self.step > 0 && self.current >= self.end)
            || (self.step < 0 && self.current <= self.end)
        {
            return None;
        }
        let val = self.current;
        self.current += self.step;
        Some(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squares_collect() {
        let result: Vec<u32> = Squares::new(5).collect();
        assert_eq!(result, vec![0, 1, 4, 9, 16]);
    }

    #[test]
    fn test_squares_sum() {
        let sum: u32 = Squares::new(4).sum();
        assert_eq!(sum, 0 + 1 + 4 + 9);
    }

    #[test]
    fn test_squares_filter() {
        let big: Vec<u32> = Squares::new(10).filter(|&x| x > 10).collect();
        assert_eq!(big, vec![16, 25, 36, 49, 64, 81]);
    }

    #[test]
    fn test_fibonacci_first_10() {
        let result: Vec<u64> = Fibonacci::new().take(10).collect();
        assert_eq!(result, vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34]);
    }

    #[test]
    fn test_fibonacci_filter() {
        let even_fibs: Vec<u64> = Fibonacci::new().take(10).filter(|x| x % 2 == 0).collect();
        assert_eq!(even_fibs, vec![0, 2, 8, 34]);
    }

    #[test]
    fn test_stepped_range() {
        let result: Vec<i32> = SteppedRange::new(0, 10, 2).collect();
        assert_eq!(result, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_stepped_range_negative() {
        let result: Vec<i32> = SteppedRange::new(10, 0, -3).collect();
        assert_eq!(result, vec![10, 7, 4, 1]);
    }

    #[test]
    fn test_zip_custom_iterators() {
        let zipped: Vec<(u32, u64)> = Squares::new(5).zip(Fibonacci::new()).collect();
        assert_eq!(zipped, vec![(0, 0), (1, 1), (4, 1), (9, 2), (16, 3)]);
    }
}

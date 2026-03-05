//! # FusedIterator for Terminated Sequences
//!
//! `FusedIterator` guarantees: once `next()` returns `None`, all future calls also return `None`.

use std::iter::FusedIterator;

/// A properly fused countdown iterator
pub struct Countdown {
    n: i32,
}

impl Countdown {
    pub fn new(n: i32) -> Self {
        Countdown { n }
    }
}

impl Iterator for Countdown {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.n <= 0 {
            return None;
        }
        let val = self.n;
        self.n -= 1;
        Some(val)
    }
}

// Declare that Countdown is fused — once None, always None
impl FusedIterator for Countdown {}

/// A badly-behaved iterator that returns Some after None (not fused)
/// This is an anti-pattern - don't do this in real code
pub struct FlickyIter {
    count: i32,
}

impl FlickyIter {
    pub fn new() -> Self {
        FlickyIter { count: 0 }
    }
}

impl Default for FlickyIter {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for FlickyIter {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.count += 1;
        if self.count == 3 {
            None // returns None once...
        } else if self.count < 6 {
            Some(self.count) // then Some again! (not fused)
        } else {
            None
        }
    }
}

/// Wraps any iterator to guarantee fused behavior
/// (This is what `.fuse()` does internally)
pub struct FusedWrapper<I> {
    inner: Option<I>,
}

impl<I> FusedWrapper<I> {
    pub fn new(iter: I) -> Self {
        FusedWrapper { inner: Some(iter) }
    }
}

impl<I: Iterator> Iterator for FusedWrapper<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.inner {
            Some(iter) => match iter.next() {
                Some(val) => Some(val),
                None => {
                    self.inner = None; // Clear on first None
                    None
                }
            },
            None => None, // Stay None forever
        }
    }
}

impl<I: Iterator> FusedIterator for FusedWrapper<I> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_countdown_fused() {
        let mut cd = Countdown::new(3);
        assert_eq!(cd.next(), Some(3));
        assert_eq!(cd.next(), Some(2));
        assert_eq!(cd.next(), Some(1));
        assert_eq!(cd.next(), None);
        assert_eq!(cd.next(), None); // stays None
    }

    #[test]
    fn test_collect_countdown() {
        let result: Vec<i32> = Countdown::new(5).collect();
        assert_eq!(result, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_flicky_is_not_fused() {
        let mut flicky = FlickyIter::new();
        assert_eq!(flicky.next(), Some(1));
        assert_eq!(flicky.next(), Some(2));
        assert_eq!(flicky.next(), None); // Returns None
        assert_eq!(flicky.next(), Some(4)); // But then Some again!
    }

    #[test]
    fn test_fuse_adapter() {
        let mut it = FlickyIter::new().fuse();
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None); // .fuse() guarantees this
    }

    #[test]
    fn test_custom_fused_wrapper() {
        let mut wrapped = FusedWrapper::new(FlickyIter::new());
        assert_eq!(wrapped.next(), Some(1));
        assert_eq!(wrapped.next(), Some(2));
        assert_eq!(wrapped.next(), None);
        assert_eq!(wrapped.next(), None); // Our wrapper also guarantees this
    }

    #[test]
    fn test_std_vec_is_fused() {
        let v = vec![1i32, 2, 3];
        let mut it = v.into_iter();
        assert_eq!(it.next(), Some(1));
        assert_eq!(it.next(), Some(2));
        assert_eq!(it.next(), Some(3));
        assert_eq!(it.next(), None);
        assert_eq!(it.next(), None); // std iterators are fused
    }

    #[test]
    fn test_countdown_sum() {
        let sum: i32 = Countdown::new(4).sum();
        assert_eq!(sum, 4 + 3 + 2 + 1);
    }
}

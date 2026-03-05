//! 284. FusedIterator for terminated sequences
//!
//! `FusedIterator` guarantees: once `next()` returns `None`, all future calls also return `None`.

use std::iter::FusedIterator;

/// A non-fused iterator (bad: returns Some after None)
struct FlickyIter {
    count: i32,
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

/// A properly fused iterator
struct Countdown {
    n: i32,
}

impl Countdown {
    fn new(n: i32) -> Self { Countdown { n } }
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

fn main() {
    // Non-fused behavior (surprising!)
    let mut flicky = FlickyIter { count: 0 };
    let results: Vec<_> = std::iter::from_fn(|| flicky.next()).take(8).collect();
    println!("Flicky results: {:?}", results);

    // Fused behavior: None then always None
    let mut cd = Countdown::new(3);
    println!("{:?} {:?} {:?} {:?} {:?}",
        cd.next(), cd.next(), cd.next(), cd.next(), cd.next());

    // All std iterators are fused
    let v = vec![1i32, 2, 3];
    let mut it = v.into_iter();
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), Some(3));
    assert_eq!(it.next(), None);
    assert_eq!(it.next(), None); // fused: stays None

    // fuse() adapter wraps any iterator to guarantee fusing
    let mut fused_flicky = FlickyIter { count: 0 }.fuse();
    let results: Vec<_> = std::iter::from_fn(|| fused_flicky.next()).take(8).collect();
    println!("Fused flicky: {:?}", results);
}

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
    fn test_fuse_adapter() {
        let mut it = FlickyIter { count: 0 }.fuse();
        while it.next().is_some() {}
        assert_eq!(it.next(), None); // .fuse() guarantees this
    }

    #[test]
    fn test_collect_countdown() {
        let result: Vec<i32> = Countdown::new(5).collect();
        assert_eq!(result, vec![5, 4, 3, 2, 1]);
    }
}

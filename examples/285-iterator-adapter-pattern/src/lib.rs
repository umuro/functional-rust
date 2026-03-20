#![allow(clippy::all)]
//! # Building Custom Iterator Adapters
//!
//! Custom adapters wrap an iterator in a struct and implement `Iterator` on it.
//! This is the same pattern used by `map`, `filter`, and `zip` in the standard library.

/// Yields every nth element starting from the first
pub struct EveryNth<I> {
    inner: I,
    n: usize,
    count: usize,
}

impl<I: Iterator> EveryNth<I> {
    pub fn new(inner: I, n: usize) -> Self {
        assert!(n > 0, "n must be positive");
        EveryNth { inner, n, count: 0 }
    }
}

impl<I: Iterator> Iterator for EveryNth<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<I::Item> {
        loop {
            let item = self.inner.next()?;
            let emit = self.count % self.n == 0;
            self.count += 1;
            if emit {
                return Some(item);
            }
        }
    }
}

/// Yields adjacent pairs (a, b) as a sliding window of 2
pub struct Pairs<I: Iterator> {
    inner: I,
    prev: Option<I::Item>,
}

impl<I: Iterator> Pairs<I>
where
    I::Item: Clone,
{
    pub fn new(mut inner: I) -> Self {
        let prev = inner.next();
        Pairs { inner, prev }
    }
}

impl<I: Iterator> Iterator for Pairs<I>
where
    I::Item: Clone,
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next()?;
        let prev = self.prev.replace(next.clone())?;
        Some((prev, next))
    }
}

/// Adapter that applies a function to each element, keeping only Some results
pub struct FilterMapWith<I, F> {
    inner: I,
    f: F,
}

impl<I, F, B> FilterMapWith<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> Option<B>,
{
    pub fn new(inner: I, f: F) -> Self {
        FilterMapWith { inner, f }
    }
}

impl<I, F, B> Iterator for FilterMapWith<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> Option<B>,
{
    type Item = B;

    fn next(&mut self) -> Option<B> {
        loop {
            match (self.f)(self.inner.next()?) {
                Some(b) => return Some(b),
                None => continue,
            }
        }
    }
}

/// Extension trait to add our adapters to all iterators
pub trait IteratorExt: Iterator + Sized {
    fn every_nth(self, n: usize) -> EveryNth<Self> {
        EveryNth::new(self, n)
    }

    fn pairs(self) -> Pairs<Self>
    where
        Self::Item: Clone,
    {
        Pairs::new(self)
    }

    fn filter_map_with<F, B>(self, f: F) -> FilterMapWith<Self, F>
    where
        F: FnMut(Self::Item) -> Option<B>,
    {
        FilterMapWith::new(self, f)
    }
}

impl<I: Iterator> IteratorExt for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_every_nth_3() {
        let result: Vec<i32> = (0..9).every_nth(3).collect();
        assert_eq!(result, vec![0, 3, 6]);
    }

    #[test]
    fn test_every_nth_2() {
        let result: Vec<i32> = (0..10).every_nth(2).collect();
        assert_eq!(result, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_pairs() {
        let result: Vec<(i32, i32)> = [1i32, 2, 3, 4].iter().copied().pairs().collect();
        assert_eq!(result, vec![(1, 2), (2, 3), (3, 4)]);
    }

    #[test]
    fn test_pairs_single_element() {
        let result: Vec<(i32, i32)> = [1i32].iter().copied().pairs().collect();
        assert!(result.is_empty());
    }

    #[test]
    fn test_every_nth_1_identity() {
        let result: Vec<i32> = [1i32, 2, 3].iter().copied().every_nth(1).collect();
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_chain_adapters() {
        let result: Vec<(i32, i32)> = (0i32..20).every_nth(2).pairs().collect();
        assert_eq!(
            result,
            vec![
                (0, 2),
                (2, 4),
                (4, 6),
                (6, 8),
                (8, 10),
                (10, 12),
                (12, 14),
                (14, 16),
                (16, 18)
            ]
        );
    }

    #[test]
    fn test_filter_map_with() {
        let result: Vec<i32> = (0..10)
            .filter_map_with(|x| if x % 2 == 0 { Some(x * 10) } else { None })
            .collect();
        assert_eq!(result, vec![0, 20, 40, 60, 80]);
    }

    #[test]
    fn test_with_standard_adapters() {
        let result: Vec<i32> = (0..20).filter(|x| x % 2 == 0).every_nth(2).collect();
        assert_eq!(result, vec![0, 4, 8, 12, 16]);
    }
}

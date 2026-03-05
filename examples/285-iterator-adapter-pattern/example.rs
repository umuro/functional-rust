//! 285. Building custom iterator adapters
//!
//! Custom adapters wrap an iterator in a struct and implement `Iterator` on it.

/// Yields every nth element starting from the first
struct EveryNth<I> {
    inner: I,
    n: usize,
    count: usize,
}

impl<I: Iterator> EveryNth<I> {
    fn new(inner: I, n: usize) -> Self {
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
struct Pairs<I: Iterator> {
    inner: I,
    prev: Option<I::Item>,
}

impl<I: Iterator> Pairs<I>
where I::Item: Clone
{
    fn new(mut inner: I) -> Self {
        let prev = inner.next();
        Pairs { inner, prev }
    }
}

impl<I: Iterator> Iterator for Pairs<I>
where I::Item: Clone
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.inner.next()?;
        let prev = self.prev.replace(next.clone())?;
        Some((prev, next))
    }
}

// Extension trait to add our adapters to all iterators
trait IteratorExt: Iterator + Sized {
    fn every_nth(self, n: usize) -> EveryNth<Self> {
        EveryNth::new(self, n)
    }
    fn pairs(self) -> Pairs<Self> where Self::Item: Clone {
        Pairs::new(self)
    }
}
impl<I: Iterator> IteratorExt for I {}

fn main() {
    // every_nth adapter
    let thirds: Vec<i32> = (0..12).every_nth(3).collect();
    println!("Every 3rd: {:?}", thirds);

    // pairs adapter
    let data = [10i32, 20, 30, 40, 50];
    let adjacent: Vec<(i32, i32)> = data.iter().copied().pairs().collect();
    println!("Adjacent pairs: {:?}", adjacent);

    // Chain custom adapters
    let result: Vec<(i32, i32)> = (0i32..20)
        .every_nth(2)
        .pairs()
        .collect();
    println!("Every 2nd, then pairs: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_every_nth_3() {
        let result: Vec<i32> = (0..9).every_nth(3).collect();
        assert_eq!(result, vec![0, 3, 6]);
    }

    #[test]
    fn test_pairs() {
        let result: Vec<(i32, i32)> = [1i32,2,3,4].iter().copied().pairs().collect();
        assert_eq!(result, vec![(1,2),(2,3),(3,4)]);
    }

    #[test]
    fn test_every_nth_1_identity() {
        let result: Vec<i32> = [1i32,2,3].iter().copied().every_nth(1).collect();
        assert_eq!(result, vec![1,2,3]);
    }
}

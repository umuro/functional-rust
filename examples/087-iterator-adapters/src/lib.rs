// 087: Iterator Adapters — build custom adapters

// Approach 1: Custom Map adapter
struct MyMap<I, F> {
    iter: I,
    f: F,
}

impl<I, F, B> Iterator for MyMap<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
{
    type Item = B;
    fn next(&mut self) -> Option<B> {
        self.iter.next().map(&mut self.f)
    }
}

// Approach 2: Custom Filter adapter
struct MyFilter<I, P> {
    iter: I,
    predicate: P,
}

impl<I, P> Iterator for MyFilter<I, P>
where
    I: Iterator,
    P: FnMut(&I::Item) -> bool,
{
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        while let Some(item) = self.iter.next() {
            if (self.predicate)(&item) { return Some(item); }
        }
        None
    }
}

// Approach 3: Custom Take adapter
struct MyTake<I> {
    iter: I,
    remaining: usize,
}

impl<I: Iterator> Iterator for MyTake<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<I::Item> {
        if self.remaining == 0 { None }
        else { self.remaining -= 1; self.iter.next() }
    }
}

// Extension trait for fluent API
trait MyIterExt: Iterator + Sized {
    fn my_map<F, B>(self, f: F) -> MyMap<Self, F> where F: FnMut(Self::Item) -> B {
        MyMap { iter: self, f }
    }
    fn my_filter<P>(self, predicate: P) -> MyFilter<Self, P> where P: FnMut(&Self::Item) -> bool {
        MyFilter { iter: self, predicate }
    }
    fn my_take(self, n: usize) -> MyTake<Self> {
        MyTake { iter: self, remaining: n }
    }
}

impl<I: Iterator> MyIterExt for I {}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_map() {
        let v: Vec<i32> = (0..5).my_map(|x| x * 2).collect();
        assert_eq!(v, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_my_filter() {
        let v: Vec<i32> = (0..5).my_filter(|x| x > &2).collect();
        assert_eq!(v, vec![3, 4]);
    }

    #[test]
    fn test_my_take() {
        let v: Vec<i32> = (0..100).my_take(3).collect();
        assert_eq!(v, vec![0, 1, 2]);
    }

    #[test]
    fn test_compose() {
        let v: Vec<i32> = (0..)
            .my_filter(|x| x % 2 == 0)
            .my_map(|x| x * x)
            .my_take(5)
            .collect();
        assert_eq!(v, vec![0, 4, 16, 36, 64]);
    }
}

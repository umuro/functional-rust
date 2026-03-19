//! # Async Generator Pattern
//! Yield values one at a time from async computations.

pub struct Generator<T> {
    items: Vec<T>,
    index: usize,
}

impl<T: Clone> Generator<T> {
    pub fn new(items: Vec<T>) -> Self {
        Self { items, index: 0 }
    }
    pub fn next(&mut self) -> Option<T> {
        if self.index < self.items.len() {
            let item = self.items[self.index].clone();
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
    pub fn reset(&mut self) {
        self.index = 0;
    }
}

impl<T: Clone> Iterator for Generator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        Generator::next(self)
    }
}

pub fn range_generator(start: i32, end: i32) -> Generator<i32> {
    Generator::new((start..end).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generator_yields() {
        let mut g = Generator::new(vec![1, 2, 3]);
        assert_eq!(g.next(), Some(1));
        assert_eq!(g.next(), Some(2));
        assert_eq!(g.next(), Some(3));
        assert_eq!(g.next(), None);
    }
    #[test]
    fn generator_reset() {
        let mut g = Generator::new(vec![1, 2]);
        g.next();
        g.next();
        g.reset();
        assert_eq!(g.next(), Some(1));
    }
    #[test]
    fn as_iterator() {
        let g = range_generator(0, 5);
        let v: Vec<_> = g.collect();
        assert_eq!(v, vec![0, 1, 2, 3, 4]);
    }
}

//! # 555. Owning References Pattern
//! Combining ownership and borrowing for efficient data access.

/// A view into an owned collection — stores offset without extra allocation
struct OwnedSlice<T> {
    data: Vec<T>,
    start: usize,
    end: usize,
}

impl<T: Clone + std::fmt::Debug> OwnedSlice<T> {
    fn new(data: Vec<T>) -> Self {
        let end = data.len();
        OwnedSlice { data, start: 0, end }
    }

    fn slice(&self, start: usize, end: usize) -> &[T] {
        &self.data[self.start + start..self.start + end.min(self.end - self.start)]
    }

    fn len(&self) -> usize { self.end - self.start }

    fn into_inner(self) -> Vec<T> { self.data }
}

/// Builder that owns source data, returns borrowed results
struct QueryBuilder {
    source: Vec<i32>,
}

impl QueryBuilder {
    fn new(data: Vec<i32>) -> Self { QueryBuilder { source: data } }

    fn filter<'a>(&'a self, pred: impl Fn(&i32) -> bool) -> impl Iterator<Item = &'a i32> {
        self.source.iter().filter(move |&&ref x| pred(x))
    }

    fn max(&self) -> Option<&i32> { self.source.iter().max() }
    fn min(&self) -> Option<&i32> { self.source.iter().min() }
}

/// Pattern: owned data + borrowed view returned
fn process_and_view(data: Vec<String>) -> (Vec<String>, Vec<usize>) {
    let lengths: Vec<usize> = data.iter().map(|s| s.len()).collect();
    (data, lengths) // own data + computed metadata
}

fn main() {
    let os = OwnedSlice::new(vec![10, 20, 30, 40, 50]);
    println!("slice [1..4]: {:?}", os.slice(1, 4));
    println!("len: {}", os.len());
    let inner = os.into_inner();
    println!("inner: {:?}", inner);

    let qb = QueryBuilder::new(vec![1, 5, 3, 7, 2, 8, 4, 6]);
    let evens: Vec<_> = qb.filter(|&x| x % 2 == 0).collect();
    println!("evens: {:?}", evens);
    println!("max: {:?}, min: {:?}", qb.max(), qb.min());

    let words = vec!["hello".to_string(), "world".to_string(), "rust".to_string()];
    let (owned_words, lengths) = process_and_view(words);
    println!("words: {:?}", owned_words);
    println!("lengths: {:?}", lengths);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_owned_slice() {
        let os = OwnedSlice::new(vec![1, 2, 3, 4, 5]);
        assert_eq!(os.slice(1, 3), &[2, 3]);
        assert_eq!(os.len(), 5);
    }

    #[test]
    fn test_query_builder() {
        let qb = QueryBuilder::new(vec![3, 1, 4, 1, 5]);
        assert_eq!(qb.max(), Some(&5));
        assert_eq!(qb.min(), Some(&1));
    }
}

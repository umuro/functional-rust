//! Type Alias Impl Trait

pub fn make_counter(start: i32, end: i32) -> impl Iterator<Item = i32> {
    start..end
}
pub fn make_even_filter(v: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().filter(|x| x % 2 == 0)
}
pub fn squares(n: u32) -> impl Iterator<Item = i64> {
    (1..=n).map(|x| (x as i64) * (x as i64))
}

pub type BoxedIter<T> = Box<dyn Iterator<Item = T>>;
pub fn range_boxed(start: i32, end: i32) -> BoxedIter<i32> {
    Box::new(start..end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        assert_eq!(make_counter(1, 4).collect::<Vec<_>>(), vec![1, 2, 3]);
    }
    #[test]
    fn test_even_filter() {
        assert_eq!(
            make_even_filter(vec![1, 2, 3, 4]).collect::<Vec<_>>(),
            vec![2, 4]
        );
    }
    #[test]
    fn test_squares() {
        assert_eq!(squares(3).collect::<Vec<_>>(), vec![1, 4, 9]);
    }
    #[test]
    fn test_boxed() {
        assert_eq!(range_boxed(1, 4).collect::<Vec<_>>(), vec![1, 2, 3]);
    }
}

#![allow(clippy::all)]
//! 278. Getting the last element
//!
//! `last()` consumes the iterator to return `Option<T>` with the final element.

#[cfg(test)]
mod tests {
    #[test]
    fn test_last_basic() {
        let v = [1i32, 2, 3, 4, 5];
        assert_eq!(v.iter().last(), Some(&5));
    }

    #[test]
    fn test_last_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(empty.iter().last(), None);
    }

    #[test]
    fn test_last_after_filter() {
        let last_even = (1i32..=10).filter(|x| x % 2 == 0).last();
        assert_eq!(last_even, Some(10));
    }

    #[test]
    fn test_last_single() {
        assert_eq!([42i32].iter().last(), Some(&42));
    }
}

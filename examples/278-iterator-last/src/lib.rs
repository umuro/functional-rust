//! 278. Getting the last element
//!
//! `last()` consumes the iterator to return `Option<T>` with the final element.

/// Returns the last element of a slice, or `None` if empty.
pub fn last<T>(slice: &[T]) -> Option<&T> {
    slice.last()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_last_basic() {
        let v = [1i32, 2, 3, 4, 5];
        assert_eq!(last(&v), Some(&5));
    }

    #[test]
    fn test_last_empty() {
        let empty: &[i32] = &[];
        assert_eq!(last(empty), None);
    }

    #[test]
    fn test_last_single() {
        assert_eq!(last(&[42i32]), Some(&42));
    }

    #[test]
    fn test_last_words() {
        let words = ["apple", "banana", "cherry"];
        assert_eq!(last(&words), Some(&"cherry"));
    }

    #[test]
    fn test_last_after_filter_iter() {
        let last_even = (1i32..=10).filter(|x| x % 2 == 0).last();
        assert_eq!(last_even, Some(10));
    }

    #[test]
    fn test_last_after_filter_collect() {
        let nums = vec![1i32, 2, 3, 4, 5];
        let evens: Vec<i32> = nums.into_iter().filter(|x| x % 2 == 0).collect();
        assert_eq!(last(&evens), Some(&4));
    }
}

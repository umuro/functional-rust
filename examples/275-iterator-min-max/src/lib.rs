//! 275. Finding extremes: min() and max()
//!
//! `min()` and `max()` return `Option<T>` — None for empty, requires `Ord`.

#[cfg(test)]
mod tests {
    #[test]
    fn test_min_max() {
        let v = [5i32, 3, 8, 1, 9, 2];
        assert_eq!(v.iter().min(), Some(&1));
        assert_eq!(v.iter().max(), Some(&9));
    }

    #[test]
    fn test_min_max_empty() {
        let empty: Vec<i32> = vec![];
        assert_eq!(empty.iter().min(), None);
        assert_eq!(empty.iter().max(), None);
    }

    #[test]
    fn test_min_by_key() {
        let words = ["hello", "hi", "world"];
        assert_eq!(words.iter().min_by_key(|w| w.len()), Some(&"hi"));
    }

    #[test]
    fn test_max_by_key() {
        let words = ["hello", "hi", "world"];
        assert_eq!(words.iter().max_by_key(|w| w.len()), Some(&"world"));
    }
}

#![allow(clippy::all)]
// 480. chars() and char-level operations

#[cfg(test)]
mod tests {
    #[test]
    fn test_count() {
        assert_eq!("café".chars().count(), 4);
        assert_eq!("café".len(), 5);
    }
    #[test]
    fn test_filter() {
        let s: String = "Hello123".chars().filter(|c| c.is_ascii_digit()).collect();
        assert_eq!(s, "123");
    }
    #[test]
    fn test_rev() {
        let s: String = "abcde".chars().rev().collect();
        assert_eq!(s, "edcba");
    }
    #[test]
    fn test_nth() {
        assert_eq!("hello".chars().nth(1), Some('e'));
    }
}

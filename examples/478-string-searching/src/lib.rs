#![allow(clippy::all)]
// 478. contains(), find(), starts_with()

#[cfg(test)]
mod tests {
    #[test]
    fn test_contains() {
        assert!("hello world".contains("world"));
        assert!(!"hello".contains("xyz"));
    }
    #[test]
    fn test_starts() {
        assert!("hello".starts_with("hel"));
        assert!(!"hello".ends_with("hel"));
    }
    #[test]
    fn test_find() {
        assert_eq!("hello".find('l'), Some(2));
        assert_eq!("hello".find('z'), None);
    }
    #[test]
    fn test_rfind() {
        assert_eq!("hello".rfind('l'), Some(3));
    }
    #[test]
    fn test_matches() {
        assert_eq!("aaabaa".matches('a').count(), 5);
    }
}

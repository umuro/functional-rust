// 475. String building patterns

#[cfg(test)]
mod tests {
    #[test]
    fn test_push() {
        let mut s = String::new();
        s.push_str("hi");
        s.push('!');
        assert_eq!(s, "hi!");
    }
    #[test]
    fn test_join() {
        assert_eq!(vec!["a", "b", "c"].join("-"), "a-b-c");
    }
    #[test]
    fn test_collect() {
        let s: String = "abc".chars().rev().collect();
        assert_eq!(s, "cba");
    }
    #[test]
    fn test_repeat() {
        assert_eq!("ha".repeat(3), "hahaha");
    }
    #[test]
    fn test_capacity() {
        let s = String::with_capacity(100);
        assert!(s.capacity() >= 100);
    }
}

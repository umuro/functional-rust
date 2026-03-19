#![allow(clippy::all)]
// 485. Efficient string concatenation

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        let a = String::from("hi");
        let b = String::from("!");
        let s = a + &b;
        assert_eq!(s, "hi!");
    }
    #[test]
    fn test_join() {
        assert_eq!(vec!["a", "b", "c"].join("-"), "a-b-c");
    }
    #[test]
    fn test_format() {
        assert_eq!(format!("{}-{}", 1, 2), "1-2");
    }
    #[test]
    fn test_collect() {
        let s: String = vec!["a", "b", "c"].join("");
        assert_eq!(s, "abc");
    }
}

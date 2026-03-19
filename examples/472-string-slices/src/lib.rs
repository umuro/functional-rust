#![allow(clippy::all)]
// 472. String slices and byte boundaries

#[cfg(test)]
mod tests {
    #[test]
    fn test_ascii() {
        assert_eq!(&"hello"[0..3], "hel");
    }
    #[test]
    fn test_safe_get() {
        assert_eq!("hello".get(1..4), Some("ell"));
        assert_eq!("hello".get(0..99), None);
    }
    #[test]
    fn test_utf8() {
        assert_eq!("café".len(), 5);
        assert_eq!("café".chars().count(), 4);
    }
    #[test]
    fn test_char_idx() {
        let v: Vec<_> = "abc".char_indices().collect();
        assert_eq!(v, vec![(0, 'a'), (1, 'b'), (2, 'c')]);
    }
}

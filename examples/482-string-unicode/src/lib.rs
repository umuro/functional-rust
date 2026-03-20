#![allow(clippy::all)]
// 482. Unicode normalization and graphemes

#[cfg(test)]
mod tests {
    #[test]
    fn test_nfc_nfd() {
        assert_ne!("caf\u{00E9}", "caf\u{0065}\u{0301}");
    }
    #[test]
    fn test_ascii_eq() {
        assert!("hello".eq_ignore_ascii_case("HELLO"));
    }
    #[test]
    fn test_is_ascii() {
        assert!("hello".is_ascii());
        assert!(!"café".is_ascii());
    }
    #[test]
    fn test_emoji() {
        let e = "\u{1F600}";
        assert_eq!(e.len(), 4);
        assert_eq!(e.chars().count(), 1);
    }
}

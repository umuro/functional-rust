// 483. UTF-8 encoding patterns

#[cfg(test)]
mod tests {
    #[test]
    fn test_encode() {
        let mut b = [0u8; 4];
        assert_eq!('A'.encode_utf8(&mut b), "A");
        assert_eq!('é'.len_utf8(), 2);
    }
    #[test]
    fn test_validate() {
        assert!(std::str::from_utf8(&[104, 105]).is_ok());
        assert!(std::str::from_utf8(&[0xFF]).is_err());
    }
    #[test]
    fn test_bom() {
        let s = "\u{FEFF}hi";
        assert_eq!(s.strip_prefix('\u{FEFF}'), Some("hi"));
    }
}

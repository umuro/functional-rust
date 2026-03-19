#![allow(clippy::all)]
// 481. bytes() and byte-level operations

#[cfg(test)]
mod tests {
    #[test]
    fn test_bytes() {
        assert_eq!("hi".bytes().collect::<Vec<_>>(), vec![104, 105]);
    }
    #[test]
    fn test_from() {
        assert_eq!(String::from_utf8(vec![104, 105]).unwrap(), "hi");
    }
    #[test]
    fn test_invalid() {
        assert!(String::from_utf8(vec![0xFF]).is_err());
    }
    #[test]
    fn test_lossy() {
        let s = String::from_utf8_lossy(&[104, 0xFF, 105]);
        assert!(s.contains('h'));
    }
}

#![allow(clippy::all)]
// 477. trim(), trim_start(), trim_end()

#[cfg(test)]
mod tests {
    #[test]
    fn test_trim() {
        assert_eq!("  hi  ".trim(), "hi");
    }
    #[test]
    fn test_trim_start() {
        assert_eq!("  hi  ".trim_start(), "hi  ");
    }
    #[test]
    fn test_trim_end() {
        assert_eq!("  hi  ".trim_end(), "  hi");
    }
    #[test]
    fn test_trim_matches() {
        assert_eq!("##hi##".trim_matches('#'), "hi");
    }
    #[test]
    fn test_trim_slice() {
        let s = "  hi  ";
        let t = s.trim();
        assert!(t.as_ptr() >= s.as_ptr());
    }
}

#![allow(clippy::all)]
// 474. format!, write!, writeln!
use std::fmt::Write as FmtWrite;

#[cfg(test)]
mod tests {
    use std::fmt::Write;
    #[test]
    fn test_align() {
        assert_eq!(format!("{:>5}", "hi"), "   hi");
        assert_eq!(format!("{:<5}", "hi"), "hi   ");
    }
    #[test]
    fn test_nums() {
        assert_eq!(format!("{:x}", 255u8), "ff");
        assert_eq!(format!("{:.2}", 3.14159f64), "3.14");
    }
    #[test]
    fn test_write() {
        let mut s = String::new();
        write!(s, "{}", 42).unwrap();
        assert_eq!(s, "42");
    }
    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", vec![1, 2, 3]), "[1, 2, 3]");
    }
}

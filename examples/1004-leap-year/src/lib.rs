#![allow(clippy::all)]
/// Idiomatic Rust: Direct boolean logic with clearest precedence
pub fn is_leap_year(year: u32) -> bool {
    (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisible_by_400() {
        assert!(is_leap_year(2000));
        assert!(is_leap_year(1600));
    }

    #[test]
    fn test_divisible_by_100_but_not_400() {
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(1800));
    }

    #[test]
    fn test_divisible_by_4_but_not_100() {
        assert!(is_leap_year(2004));
        assert!(is_leap_year(2024));
    }

    #[test]
    fn test_not_divisible_by_4() {
        assert!(!is_leap_year(2001));
        assert!(!is_leap_year(2003));
        assert!(!is_leap_year(2100));
    }

    #[test]
    fn test_edge_cases() {
        assert!(is_leap_year(4)); // Year 4 is a leap year
        assert!(!is_leap_year(2)); // Year 2 is NOT a leap year
    }
}

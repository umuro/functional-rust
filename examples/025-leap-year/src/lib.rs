#![allow(clippy::all)]
// Gregorian leap year rule: divisible by 400, or by 4 but not by 100
pub fn is_leap_year(year: u32) -> bool {
    (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisible_by_400_is_leap() {
        assert!(is_leap_year(2000));
        assert!(is_leap_year(1600));
    }

    #[test]
    fn test_divisible_by_100_not_400_is_not_leap() {
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(1800));
    }

    #[test]
    fn test_divisible_by_4_not_100_is_leap() {
        assert!(is_leap_year(2004));
        assert!(is_leap_year(2024));
    }

    #[test]
    fn test_not_divisible_by_4_is_not_leap() {
        assert!(!is_leap_year(2001));
        assert!(!is_leap_year(2003));
    }
}

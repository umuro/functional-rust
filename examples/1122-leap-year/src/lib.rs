#![allow(dead_code)]
//! Leap Year — Gregorian calendar rule
//! See example.ml for OCaml reference

/// Returns true if `year` is a leap year in the Gregorian calendar.
/// Rule: divisible by 400, OR (divisible by 4 AND NOT divisible by 100).
///
/// Mirrors OCaml:
///   `year mod 400 = 0 || (year mod 4 = 0 && year mod 100 <> 0)`
pub fn is_leap_year(year: i32) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

/// Functional/recursive variant: decompose the rule into named predicates.
/// Shows the three-part logic in a way that mirrors the Gregorian definition.
pub fn is_leap_year_explicit(year: i32) -> bool {
    let divisible_by_4 = year % 4 == 0;
    let divisible_by_100 = year % 100 == 0;
    let divisible_by_400 = year % 400 == 0;
    divisible_by_400 || (divisible_by_4 && !divisible_by_100)
}

/// Number of days in a year — 366 for leap years, 365 otherwise.
pub fn days_in_year(year: i32) -> u32 {
    if is_leap_year(year) {
        366
    } else {
        365
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regular_non_leap() {
        // Not divisible by 4
        assert!(!is_leap_year(1997));
        assert!(!is_leap_year(2001));
    }

    #[test]
    fn test_regular_leap() {
        // Divisible by 4, not by 100
        assert!(is_leap_year(1996));
        assert!(is_leap_year(2004));
        assert!(is_leap_year(2024));
    }

    #[test]
    fn test_century_not_leap() {
        // Divisible by 100 but not 400
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(1800));
        assert!(!is_leap_year(2100));
    }

    #[test]
    fn test_century_leap() {
        // Divisible by 400
        assert!(is_leap_year(1600));
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2400));
    }

    #[test]
    fn test_explicit_matches_idiomatic() {
        for year in [1900, 1996, 2000, 2001, 2024, 2100] {
            assert_eq!(is_leap_year(year), is_leap_year_explicit(year));
        }
    }

    #[test]
    fn test_days_in_year() {
        assert_eq!(days_in_year(2000), 366);
        assert_eq!(days_in_year(1900), 365);
        assert_eq!(days_in_year(2024), 366);
        assert_eq!(days_in_year(2023), 365);
    }
}

#![allow(clippy::all)]
//! Leap Year Validator
//!
//! A year is a leap year if:
//! - It is divisible by 400, OR
//! - It is divisible by 4 AND not divisible by 100
//!
//! Examples:
//! - 2000: leap (divisible by 400)
//! - 1900: not leap (divisible by 100 but not 400)
//! - 2004: leap (divisible by 4 but not 100)
//! - 2001: not leap (not divisible by 4)
//!
//! Note: We use the `%` operator directly instead of `is_multiple_of()` to match
//! the idiomatic style of the OCaml implementation and because the modulo operator
//! is more familiar to a broader audience of programmers.

/// Determines if a year is a leap year (idiomatic Rust expression).
///
/// # Arguments
/// * `year` - The year to check
///
/// # Returns
/// `true` if the year is a leap year, `false` otherwise
///
/// # Examples
/// ```no_run
/// use leap_year::is_leap_year;
/// assert!(is_leap_year(2000));
/// assert!(!is_leap_year(1900));
/// assert!(is_leap_year(2004));
/// assert!(!is_leap_year(2001));
/// ```
pub fn is_leap_year(year: u32) -> bool {
    (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
}

/// Determines if a year is a leap year using guard clauses.
///
/// This is an alternative implementation that demonstrates a different
/// coding style using early returns and guard clauses.
///
/// # Arguments
/// * `year` - The year to check
///
/// # Returns
/// `true` if the year is a leap year, `false` otherwise
///
/// # Examples
/// ```no_run
/// use leap_year::is_leap_year_guards;
/// assert!(is_leap_year_guards(2000));
/// assert!(!is_leap_year_guards(1900));
/// ```
pub fn is_leap_year_guards(year: u32) -> bool {
    // Divisible by 400 is always a leap year
    if year % 400 == 0 {
        return true;
    }
    // Divisible by 100 (but not 400 due to above guard) is never a leap year
    if year % 100 == 0 {
        return false;
    }
    // Divisible by 4 is a leap year
    year % 4 == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for divisible by 400 (always leap)
    #[test]
    fn test_divisible_by_400() {
        assert!(is_leap_year(2000));
        assert!(is_leap_year(2400));
        assert!(is_leap_year(1600));
    }

    // Tests for divisible by 100 but not 400 (never leap)
    #[test]
    fn test_divisible_by_100_not_400() {
        assert!(!is_leap_year(1900));
        assert!(!is_leap_year(2100));
        assert!(!is_leap_year(1800));
    }

    // Tests for divisible by 4 but not 100 (always leap)
    #[test]
    fn test_divisible_by_4_not_100() {
        assert!(is_leap_year(2004));
        assert!(is_leap_year(2008));
        assert!(is_leap_year(2012));
        assert!(is_leap_year(2016));
    }

    // Tests for non-leap years (not divisible by 4)
    #[test]
    fn test_non_leap_years() {
        assert!(!is_leap_year(2001));
        assert!(!is_leap_year(2002));
        assert!(!is_leap_year(2003));
        assert!(!is_leap_year(2017));
    }

    // Guard clause implementation should match the expression version
    #[test]
    fn test_both_implementations_match() {
        for year in [2000, 1900, 2004, 2001, 1600, 2100, 2008, 2017].iter() {
            assert_eq!(
                is_leap_year(*year),
                is_leap_year_guards(*year),
                "Implementations diverged for year {}",
                year
            );
        }
    }

    // Edge cases
    #[test]
    fn test_edge_cases() {
        assert!(!is_leap_year(1));
        assert!(!is_leap_year(3));
        assert!(is_leap_year(4));
        assert!(is_leap_year(400));
    }
}

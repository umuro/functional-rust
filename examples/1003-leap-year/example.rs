//! Leap Year Example - Demonstration
//!
//! This file shows how to use the leap year functions with practical examples.

/// Determines if a year is a leap year (idiomatic Rust expression).
pub fn is_leap_year(year: u32) -> bool {
    (year % 400 == 0) || (year % 4 == 0 && year % 100 != 0)
}

/// Determines if a year is a leap year using guard clauses.
pub fn is_leap_year_guards(year: u32) -> bool {
    if year % 400 == 0 {
        return true;
    }
    if year % 100 == 0 {
        return false;
    }
    year % 4 == 0
}

fn main() {
    println!("=== Leap Year Validator ===\n");

    let test_years = vec![
        (2000, "divisible by 400"),
        (1900, "divisible by 100, not 400"),
        (2004, "divisible by 4, not 100"),
        (2001, "not divisible by 4"),
        (2024, "divisible by 4, not 100"),
        (2100, "divisible by 100, not 400"),
    ];

    println!("Using idiomatic expression:");
    for (year, description) in &test_years {
        let result = is_leap_year(*year);
        println!("  {} ({:?}): {}", year, description, if result { "LEAP" } else { "NOT LEAP" });
    }

    println!("\nUsing guard clause implementation:");
    for (year, description) in &test_years {
        let result = is_leap_year_guards(*year);
        println!("  {} ({:?}): {}", year, description, if result { "LEAP" } else { "NOT LEAP" });
    }

    println!("\n=== Verification ===");
    let all_match = test_years.iter().all(|(year, _)| {
        is_leap_year(*year) == is_leap_year_guards(*year)
    });
    println!("Both implementations match: {}", all_match);
}

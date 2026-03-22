#![allow(dead_code)]

/// Returns true if `year` is a leap year in the Gregorian calendar.
/// Rule: divisible by 400, OR (divisible by 4 AND NOT divisible by 100).
pub fn is_leap_year(year: i32) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

/// Explicit decomposition: names each sub-predicate for clarity.
pub fn is_leap_year_explicit(year: i32) -> bool {
    let divisible_by_4 = year % 4 == 0;
    let divisible_by_100 = year % 100 == 0;
    let divisible_by_400 = year % 400 == 0;
    divisible_by_400 || (divisible_by_4 && !divisible_by_100)
}

/// Number of days in a year.
pub fn days_in_year(year: i32) -> u32 {
    if is_leap_year(year) { 366 } else { 365 }
}

fn main() {
    let years = [1900, 1996, 2000, 2001, 2024, 2100];
    for year in years {
        println!(
            "{}: leap={}, days={}",
            year,
            is_leap_year(year),
            days_in_year(year)
        );
    }
}

/* Output:
   1900: leap=false, days=365
   1996: leap=true, days=366
   2000: leap=true, days=366
   2001: leap=false, days=365
   2024: leap=true, days=366
   2100: leap=false, days=365
*/

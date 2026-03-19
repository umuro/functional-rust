#![allow(clippy::all)]
//! # Variants — Days of the Week
//!
//! OCaml variants map cleanly to Rust enums. Both are algebraic data types
//! with exhaustive pattern matching enforced by the compiler.

// ---------------------------------------------------------------------------
// Approach A: Idiomatic Rust — enum with methods via impl block
// ---------------------------------------------------------------------------

/// Days of the week as a simple C-like enum.
///
/// We derive common traits that OCaml variants get implicitly:
/// - `Debug` for printing
/// - `Clone, Copy` because this is a simple fieldless enum
/// - `PartialEq, Eq` for equality comparison
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Day {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}

impl Day {
    /// Human-readable name.
    pub fn name(self) -> &'static str {
        match self {
            Day::Sun => "Sunday",
            Day::Mon => "Monday",
            Day::Tue => "Tuesday",
            Day::Wed => "Wednesday",
            Day::Thu => "Thursday",
            Day::Fri => "Friday",
            Day::Sat => "Saturday",
        }
    }

    /// Is this a weekend day?
    pub fn is_weekend(self) -> bool {
        matches!(self, Day::Sun | Day::Sat)
    }

    /// Next day of the week (wraps around).
    pub fn next(self) -> Day {
        match self {
            Day::Sun => Day::Mon,
            Day::Mon => Day::Tue,
            Day::Tue => Day::Wed,
            Day::Wed => Day::Thu,
            Day::Thu => Day::Fri,
            Day::Fri => Day::Sat,
            Day::Sat => Day::Sun,
        }
    }
}

// ---------------------------------------------------------------------------
// Approach B: Functional style — free functions with pattern matching
// ---------------------------------------------------------------------------

/// Mirrors the OCaml `day_name` function — a standalone function,
/// not a method.
pub fn day_name(d: Day) -> &'static str {
    // Identical logic but as a free function rather than a method.
    // In OCaml all functions on types are free functions.
    d.name()
}

pub fn is_weekend(d: Day) -> bool {
    d.is_weekend()
}

pub fn next_day(d: Day) -> Day {
    d.next()
}

// ---------------------------------------------------------------------------
// Approach C: Numeric representation — using discriminants
// ---------------------------------------------------------------------------

impl Day {
    /// Convert from a 0-based index (Sun=0 .. Sat=6).
    pub fn from_index(i: u8) -> Option<Day> {
        match i {
            0 => Some(Day::Sun),
            1 => Some(Day::Mon),
            2 => Some(Day::Tue),
            3 => Some(Day::Wed),
            4 => Some(Day::Thu),
            5 => Some(Day::Fri),
            6 => Some(Day::Sat),
            _ => None,
        }
    }

    /// Convert to a 0-based index.
    pub fn to_index(self) -> u8 {
        self as u8
    }

    /// Next day using arithmetic modulo — avoids exhaustive match.
    pub fn next_arithmetic(self) -> Day {
        Day::from_index((self.to_index() + 1) % 7).unwrap()
    }
}

/// Display trait for pretty-printing.
impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_names() {
        assert_eq!(Day::Sun.name(), "Sunday");
        assert_eq!(Day::Wed.name(), "Wednesday");
        assert_eq!(Day::Sat.name(), "Saturday");
    }

    #[test]
    fn test_is_weekend() {
        assert!(Day::Sun.is_weekend());
        assert!(Day::Sat.is_weekend());
        assert!(!Day::Mon.is_weekend());
        assert!(!Day::Wed.is_weekend());
        assert!(!Day::Fri.is_weekend());
    }

    #[test]
    fn test_next_day() {
        assert_eq!(Day::Sun.next(), Day::Mon);
        assert_eq!(Day::Wed.next(), Day::Thu);
        assert_eq!(Day::Sat.next(), Day::Sun); // wraps around
    }

    #[test]
    fn test_next_full_cycle() {
        // Going through all 7 days should return to start
        let mut d = Day::Mon;
        for _ in 0..7 {
            d = d.next();
        }
        assert_eq!(d, Day::Mon);
    }

    #[test]
    fn test_arithmetic_next() {
        assert_eq!(Day::Sun.next_arithmetic(), Day::Mon);
        assert_eq!(Day::Sat.next_arithmetic(), Day::Sun);
        // Should agree with pattern-match version for all days
        for i in 0..7 {
            let d = Day::from_index(i).unwrap();
            assert_eq!(d.next(), d.next_arithmetic());
        }
    }

    #[test]
    fn test_from_index_invalid() {
        assert_eq!(Day::from_index(7), None);
        assert_eq!(Day::from_index(255), None);
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Day::Fri), "Friday");
    }
}

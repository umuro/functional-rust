use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum ParsePositiveError { Empty, InvalidNumber(String), NotPositive(i64) }

impl fmt::Display for ParsePositiveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => write!(f, "empty input"),
            Self::InvalidNumber(s) => write!(f, "not a number: {s}"),
            Self::NotPositive(n) => write!(f, "{n} is not positive"),
        }
    }
}

struct PositiveInt(u64);

impl FromStr for PositiveInt {
    type Err = ParsePositiveError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() { return Err(ParsePositiveError::Empty); }
        let n: i64 = s.parse().map_err(|_| ParsePositiveError::InvalidNumber(s.to_string()))?;
        if n <= 0 { return Err(ParsePositiveError::NotPositive(n)); }
        Ok(PositiveInt(n as u64))
    }
}

fn main() {
    for s in &["42", "-1", "abc", "", "100"] {
        match s.parse::<PositiveInt>() {
            Ok(PositiveInt(n)) => println!("Parsed: {n}"),
            Err(e) => println!("Error: {e}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn valid() { assert_eq!("42".parse::<PositiveInt>().unwrap().0, 42); }
    #[test] fn rejects_zero() { assert_eq!("0".parse::<PositiveInt>().unwrap_err(), ParsePositiveError::NotPositive(0)); }
    #[test] fn rejects_empty() { assert_eq!("".parse::<PositiveInt>().unwrap_err(), ParsePositiveError::Empty); }
    #[test] fn rejects_text() { assert!(matches!("abc".parse::<PositiveInt>().unwrap_err(), ParsePositiveError::InvalidNumber(_))); }
}

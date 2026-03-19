#![allow(clippy::all)]
//! # Error Propagation in Closures
//!
//! `?` in closures requires the closure to return `Result`/`Option`.

/// Parse number from string
pub fn parse_number(s: &str) -> Result<i32, String> {
    s.trim()
        .parse::<i32>()
        .map_err(|_| format!("not a number: '{}'", s))
}

/// Collect results - short-circuits on first error
pub fn parse_all(inputs: &[&str]) -> Result<Vec<i32>, String> {
    inputs.iter().map(|s| parse_number(s)).collect()
}

/// Filter and keep only valid parses (drops errors)
pub fn parse_valid(inputs: &[&str]) -> Vec<i32> {
    inputs
        .iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect()
}

/// Try fold for short-circuit accumulation
pub fn sum_all(inputs: &[&str]) -> Result<i32, String> {
    inputs
        .iter()
        .try_fold(0i32, |acc, s| Ok(acc + parse_number(s)?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_all_ok() {
        let result = parse_all(&["1", "2", "3"]);
        assert_eq!(result.unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_parse_all_err() {
        let result = parse_all(&["1", "bad", "3"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_valid() {
        let result = parse_valid(&["1", "bad", "3"]);
        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    fn test_sum_all_ok() {
        let result = sum_all(&["1", "2", "3"]);
        assert_eq!(result.unwrap(), 6);
    }

    #[test]
    fn test_sum_all_err() {
        let result = sum_all(&["1", "x", "3"]);
        assert!(result.is_err());
    }
}

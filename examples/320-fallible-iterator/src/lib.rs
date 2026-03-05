//! # Fallible Iterator
//!
//! Iterators over Results with collect short-circuiting and best-effort patterns.

/// Parse a single integer
pub fn parse_int(s: &str) -> Result<i64, String> {
    s.parse::<i64>().map_err(|_| format!("cannot parse: {s}"))
}

/// Parse all - short-circuits on first error
pub fn parse_all(inputs: &[&str]) -> Result<Vec<i64>, String> {
    inputs.iter().map(|s| parse_int(s)).collect()
}

/// Parse best effort - skip errors
pub fn parse_best_effort(inputs: &[&str]) -> Vec<i64> {
    inputs.iter().filter_map(|s| parse_int(s).ok()).collect()
}

/// Parse with fallback value for errors
pub fn parse_with_default(inputs: &[&str], default: i64) -> Vec<i64> {
    inputs.iter().map(|s| parse_int(s).unwrap_or(default)).collect()
}

/// Partition into successes and failures
pub fn parse_partition(inputs: &[&str]) -> (Vec<i64>, Vec<String>) {
    let (oks, errs): (Vec<_>, Vec<_>) = inputs
        .iter()
        .map(|s| parse_int(s))
        .partition(Result::is_ok);
    let nums: Vec<i64> = oks.into_iter().map(Result::unwrap).collect();
    let errors: Vec<String> = errs.into_iter().map(Result::unwrap_err).collect();
    (nums, errors)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_valid() {
        assert_eq!(parse_all(&["1", "2", "3"]), Ok(vec![1, 2, 3]));
    }

    #[test]
    fn test_short_circuits() {
        assert!(parse_all(&["1", "bad", "3"]).is_err());
    }

    #[test]
    fn test_best_effort() {
        assert_eq!(parse_best_effort(&["1", "bad", "3"]), vec![1, 3]);
    }

    #[test]
    fn test_empty_ok() {
        assert_eq!(parse_all(&[]), Ok(vec![]));
    }

    #[test]
    fn test_with_default() {
        assert_eq!(parse_with_default(&["1", "bad", "3"], 0), vec![1, 0, 3]);
    }

    #[test]
    fn test_partition() {
        let (nums, errs) = parse_partition(&["1", "bad", "3", "oops"]);
        assert_eq!(nums, vec![1, 3]);
        assert_eq!(errs.len(), 2);
    }
}

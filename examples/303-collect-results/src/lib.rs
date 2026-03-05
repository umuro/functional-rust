//! # Collecting Iterator<Result<T>> into Result<Vec<T>>
//!
//! `collect::<Result<Vec<T>,E>>()` short-circuits on first Err.

/// Parse all strings - fails on first error
pub fn parse_all(inputs: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> {
    inputs.iter().map(|s| s.parse::<i32>()).collect()
}

/// Sum parsed numbers - returns error if any parse fails
pub fn sum_all(inputs: &[&str]) -> Result<i32, std::num::ParseIntError> {
    let nums: Vec<i32> = parse_all(inputs)?;
    Ok(nums.iter().sum())
}

/// Process and transform - all-or-nothing
pub fn double_all(inputs: &[&str]) -> Result<Vec<i32>, std::num::ParseIntError> {
    inputs
        .iter()
        .map(|s| s.parse::<i32>().map(|n| n * 2))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_all_ok() {
        let result = parse_all(&["1", "2", "3"]);
        assert_eq!(result.unwrap(), vec![1, 2, 3]);
    }

    #[test]
    fn test_collect_with_err() {
        let result = parse_all(&["1", "bad", "3"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_sum_all_ok() {
        let result = sum_all(&["10", "20", "30"]);
        assert_eq!(result.unwrap(), 60);
    }

    #[test]
    fn test_sum_all_err() {
        let result = sum_all(&["10", "x", "30"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_double_all() {
        let result = double_all(&["1", "2", "3"]);
        assert_eq!(result.unwrap(), vec![2, 4, 6]);
    }

    #[test]
    fn test_empty_input() {
        let result = parse_all(&[]);
        assert_eq!(result.unwrap(), Vec::<i32>::new());
    }
}

#![allow(clippy::all)]
//! # Option::transpose() — Collecting Optional Results
//!
//! Convert `Option<Result<T, E>>` into `Result<Option<T>, E>`.

use std::collections::HashMap;

/// Lookup a key and parse its value
pub fn lookup_and_parse(
    map: &HashMap<&str, &str>,
    key: &str,
) -> Result<Option<i32>, std::num::ParseIntError> {
    map.get(key).map(|s| s.parse::<i32>()).transpose()
}

/// Filter and parse optional values
pub fn parse_optional_values(
    inputs: Vec<Option<&str>>,
) -> Result<Vec<i32>, std::num::ParseIntError> {
    inputs
        .into_iter()
        .filter_map(|opt| opt.map(|s| s.parse::<i32>()))
        .collect::<Result<Vec<_>, _>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_ok_transpose() {
        let v: Option<Result<i32, &str>> = Some(Ok(5));
        assert_eq!(v.transpose(), Ok(Some(5)));
    }

    #[test]
    fn test_some_err_transpose() {
        let v: Option<Result<i32, &str>> = Some(Err("fail"));
        assert_eq!(v.transpose(), Err("fail"));
    }

    #[test]
    fn test_none_transpose() {
        let v: Option<Result<i32, &str>> = None;
        assert_eq!(v.transpose(), Ok(None));
    }

    #[test]
    fn test_lookup_found() {
        let mut map = HashMap::new();
        map.insert("port", "8080");
        assert_eq!(lookup_and_parse(&map, "port").unwrap(), Some(8080));
    }

    #[test]
    fn test_lookup_missing() {
        let map: HashMap<&str, &str> = HashMap::new();
        assert_eq!(lookup_and_parse(&map, "port").unwrap(), None);
    }

    #[test]
    fn test_lookup_invalid() {
        let mut map = HashMap::new();
        map.insert("port", "bad");
        assert!(lookup_and_parse(&map, "port").is_err());
    }

    #[test]
    fn test_parse_optional_values() {
        let inputs = vec![Some("1"), None, Some("2")];
        let result = parse_optional_values(inputs);
        assert_eq!(result.unwrap(), vec![1, 2]);
    }
}

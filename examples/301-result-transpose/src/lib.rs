//! # Result::transpose() — Flipping Nested Types
//!
//! Convert `Result<Option<T>, E>` into `Option<Result<T, E>>` — or back again.

/// Parse an optional string into a number
pub fn maybe_parse(s: Option<&str>) -> Result<Option<i32>, std::num::ParseIntError> {
    s.map(|s| s.parse::<i32>()).transpose()
}

/// Result transpose: Ok(Some(v)) -> Some(Ok(v))
pub fn result_transpose<T, E>(r: Result<Option<T>, E>) -> Option<Result<T, E>> {
    r.transpose()
}

/// Option transpose: Some(Ok(v)) -> Ok(Some(v))
pub fn option_transpose<T, E>(o: Option<Result<T, E>>) -> Result<Option<T>, E> {
    o.transpose()
}

/// Practical: parse an optional config value
pub fn parse_optional_config(
    config_val: Option<&str>,
) -> Result<Option<i32>, std::num::ParseIntError> {
    config_val.map(|s| s.parse::<i32>()).transpose()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_result_transpose_ok_some() {
        let r: Result<Option<i32>, &str> = Ok(Some(42));
        assert_eq!(r.transpose(), Some(Ok(42)));
    }

    #[test]
    fn test_result_transpose_ok_none() {
        let r: Result<Option<i32>, &str> = Ok(None);
        assert_eq!(r.transpose(), None);
    }

    #[test]
    fn test_result_transpose_err() {
        let r: Result<Option<i32>, &str> = Err("bad");
        assert_eq!(r.transpose(), Some(Err("bad")));
    }

    #[test]
    fn test_option_transpose_some_ok() {
        let o: Option<Result<i32, &str>> = Some(Ok(5));
        assert_eq!(o.transpose(), Ok(Some(5)));
    }

    #[test]
    fn test_option_transpose_some_err() {
        let o: Option<Result<i32, &str>> = Some(Err("fail"));
        assert_eq!(o.transpose(), Err("fail"));
    }

    #[test]
    fn test_option_transpose_none() {
        let o: Option<Result<i32, &str>> = None;
        assert_eq!(o.transpose(), Ok(None));
    }

    #[test]
    fn test_parse_optional_config_some() {
        let result = parse_optional_config(Some("42"));
        assert_eq!(result.unwrap(), Some(42));
    }

    #[test]
    fn test_parse_optional_config_none() {
        let result = parse_optional_config(None);
        assert_eq!(result.unwrap(), None);
    }
}

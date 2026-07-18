#![allow(clippy::all)]
// Result<T, E> basics: typed errors instead of sentinel values or exceptions.
pub fn parse_positive(s: &str) -> Result<u32, String> {
    match s.parse::<i64>() {
        Err(_) => Err(format!("not an integer: {}", s)),
        Ok(n) if n <= 0 => Err(format!("expected positive, got {}", n)),
        Ok(n) => Ok(n as u32),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_positive_ok() {
        assert_eq!(parse_positive("42"), Ok(42));
    }

    #[test]
    fn test_parse_positive_non_integer() {
        assert_eq!(parse_positive("abc"), Err("not an integer: abc".to_string()));
    }

    #[test]
    fn test_parse_positive_negative() {
        assert_eq!(parse_positive("-5"), Err("expected positive, got -5".to_string()));
    }

    #[test]
    fn test_parse_positive_zero() {
        assert_eq!(parse_positive("0"), Err("expected positive, got 0".to_string()));
    }

    #[test]
    fn test_ok_to_option() {
        assert_eq!(parse_positive("10").ok(), Some(10));
        assert_eq!(parse_positive("x").ok(), None);
    }
}

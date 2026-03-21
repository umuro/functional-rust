#![allow(dead_code)]
#![allow(clippy::all)]
// 1007: Result Combinators
// and_then, or_else, map, map_err, unwrap_or_else

fn parse_int(s: &str) -> Result<i64, String> {
    s.parse::<i64>()
        .map_err(|e| format!("not an int: {} ({})", s, e))
}

fn double_if_positive(n: i64) -> Result<i64, String> {
    if n > 0 {
        Ok(n * 2)
    } else {
        Err("must be positive".into())
    }
}

// Approach 1: Chaining with and_then (flatmap/bind)
fn process_chain(s: &str) -> Result<String, String> {
    parse_int(s)
        .and_then(double_if_positive)
        .map(|n| n.to_string())
}

// Approach 2: Using map, map_err, or_else, unwrap_or_else
fn process_with_fallback(s: &str) -> String {
    parse_int(s)
        .and_then(double_if_positive)
        .map(|n| n.to_string())
        .map_err(|e| format!("FALLBACK: {}", e))
        .unwrap_or_else(|e| e)
}

fn process_or_else(s: &str) -> Result<i64, String> {
    parse_int(s).and_then(double_if_positive).or_else(|_| Ok(0)) // fallback to 0 on any error
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_and_then_success() {
        assert_eq!(process_chain("5"), Ok("10".to_string()));
    }

    #[test]
    fn test_and_then_negative() {
        assert_eq!(process_chain("-3"), Err("must be positive".to_string()));
    }

    #[test]
    fn test_and_then_parse_fail() {
        assert!(process_chain("abc").is_err());
    }

    #[test]
    fn test_map() {
        let result: Result<i64, String> = Ok(5);
        assert_eq!(result.map(|n| n * 2), Ok(10));
    }

    #[test]
    fn test_map_err() {
        let result: Result<i64, &str> = Err("low");
        assert_eq!(result.map_err(|e| e.to_uppercase()), Err("LOW".to_string()));
    }

    #[test]
    fn test_or_else() {
        assert_eq!(process_or_else("-1"), Ok(0));
        assert_eq!(process_or_else("5"), Ok(10));
    }

    #[test]
    fn test_unwrap_or_else() {
        let result: Result<i64, String> = Err("fail".into());
        assert_eq!(result.unwrap_or_else(|_| 99), 99);

        let result: Result<i64, String> = Ok(42);
        assert_eq!(result.unwrap_or_else(|_| 99), 42);
    }

    #[test]
    fn test_fallback_string() {
        assert_eq!(process_with_fallback("5"), "10");
        assert!(process_with_fallback("abc").starts_with("FALLBACK"));
    }
}

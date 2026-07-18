#![allow(clippy::all)]
// Result::and_then (bind): sequences fallible steps, short-circuiting on the first Err.
pub fn parse_int(s: &str) -> Result<i32, String> {
    s.parse::<i32>().map_err(|_| format!("invalid integer: {}", s))
}

pub fn safe_div(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

pub fn pipeline(s: &str, divisor: i32) -> Result<String, String> {
    parse_int(s).and_then(|n| safe_div(n, divisor)).map(|n| n.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_ok() {
        assert_eq!(pipeline("100", 5), Ok("20".to_string()));
    }

    #[test]
    fn test_pipeline_parse_fails() {
        assert_eq!(pipeline("abc", 5), Err("invalid integer: abc".to_string()));
    }

    #[test]
    fn test_pipeline_div_fails() {
        assert_eq!(pipeline("100", 0), Err("division by zero".to_string()));
    }
}

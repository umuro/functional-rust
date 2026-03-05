/// 755: Testing Error Cases and Unwrap Discipline

// ── Error types ────────────────────────────────────────────────────────────────

#[derive(Debug, PartialEq, Clone)]
pub enum ParseError {
    Empty,
    TooLong { len: usize, max: usize },
    InvalidChar { ch: char, pos: usize },
    OutOfRange { value: i64, min: i64, max: i64 },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::Empty                    => write!(f, "input is empty"),
            ParseError::TooLong { len, max }     => write!(f, "too long: {} > {}", len, max),
            ParseError::InvalidChar { ch, pos }  => write!(f, "invalid char {:?} at {}", ch, pos),
            ParseError::OutOfRange { value, min, max } =>
                write!(f, "{} out of range [{}, {}]", value, min, max),
        }
    }
}

// ── Function under test ────────────────────────────────────────────────────────

pub fn parse_positive(s: &str) -> Result<u32, ParseError> {
    if s.is_empty() {
        return Err(ParseError::Empty);
    }
    if s.len() > 10 {
        return Err(ParseError::TooLong { len: s.len(), max: 10 });
    }
    for (pos, ch) in s.char_indices() {
        if !ch.is_ascii_digit() {
            return Err(ParseError::InvalidChar { ch, pos });
        }
    }
    let n: u64 = s.parse().unwrap(); // safe: all digits verified
    if n == 0 || n > u32::MAX as u64 {
        return Err(ParseError::OutOfRange { value: n as i64, min: 1, max: u32::MAX as i64 });
    }
    Ok(n as u32)
}

pub fn divide(a: i64, b: i64) -> Result<i64, &'static str> {
    if b == 0 { Err("cannot divide by zero") } else { Ok(a / b) }
}

pub fn head<T: Clone>(v: &[T]) -> Result<T, &'static str> {
    v.first().cloned().ok_or("slice is empty")
}

fn main() {
    println!("{:?}", parse_positive("42"));
    println!("{:?}", parse_positive(""));
    println!("{:?}", parse_positive("12345678901"));
    println!("{:?}", parse_positive("12x45"));
    println!("{:?}", divide(10, 3));
    println!("{:?}", divide(10, 0));
}

// ── Tests ──────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── Happy path ────────────────────────────────────────────────────────────

    #[test]
    fn parse_positive_valid() {
        // Use expect() — gives context on failure
        let n = parse_positive("42").expect("'42' is a valid positive integer");
        assert_eq!(n, 42);
    }

    #[test]
    fn parse_positive_boundary_values() {
        assert_eq!(parse_positive("1").expect("1 is valid"), 1);
        // u32::MAX = 4294967295 (10 chars — valid)
        let max_str = u32::MAX.to_string();
        assert_eq!(parse_positive(&max_str).expect("u32::MAX is valid"), u32::MAX);
    }

    // ── Error paths — variant-specific ────────────────────────────────────────

    #[test]
    fn parse_empty_returns_empty_error() {
        assert_eq!(parse_positive(""), Err(ParseError::Empty));
    }

    #[test]
    fn parse_too_long_returns_correct_lengths() {
        let long = "1".repeat(11);
        match parse_positive(&long) {
            Err(ParseError::TooLong { len: 11, max: 10 }) => {}
            other => panic!("expected TooLong, got {:?}", other),
        }
    }

    #[test]
    fn parse_invalid_char_reports_position() {
        match parse_positive("12x45") {
            Err(ParseError::InvalidChar { ch: 'x', pos: 2 }) => {}
            other => panic!("expected InvalidChar('x',2), got {:?}", other),
        }
    }

    #[test]
    fn parse_zero_is_out_of_range() {
        match parse_positive("0") {
            Err(ParseError::OutOfRange { value: 0, .. }) => {}
            other => panic!("expected OutOfRange(0), got {:?}", other),
        }
    }

    // ── is_err / is_ok checks ─────────────────────────────────────────────────

    #[test]
    fn parse_non_digit_is_err() {
        assert!(parse_positive("abc").is_err());
        assert!(parse_positive("-1").is_err());
        assert!(parse_positive("1.5").is_err());
    }

    // ── should_panic ──────────────────────────────────────────────────────────

    #[test]
    #[should_panic(expected = "called `Result::unwrap()`")]
    fn unwrap_on_err_panics() {
        parse_positive("").unwrap();
    }

    #[test]
    #[should_panic(expected = "parse_positive should succeed")]
    fn expect_gives_context_on_failure() {
        parse_positive("not-a-number")
            .expect("parse_positive should succeed");
    }

    // ── unwrap_or_else discipline ──────────────────────────────────────────────

    #[test]
    fn use_unwrap_or_else_for_defaults() {
        let n = parse_positive("bad").unwrap_or_else(|_| 0);
        assert_eq!(n, 0);
    }

    #[test]
    fn divide_by_zero_is_err() {
        assert!(divide(10, 0).is_err());
        assert_eq!(divide(10, 0), Err("cannot divide by zero"));
    }

    #[test]
    fn head_empty_is_err() {
        let v: Vec<i32> = vec![];
        assert_eq!(head(&v), Err("slice is empty"));
    }

    #[test]
    fn head_non_empty_is_ok() {
        let v = vec![42, 1, 2];
        assert_eq!(head(&v).expect("non-empty slice has a head"), 42);
    }
}

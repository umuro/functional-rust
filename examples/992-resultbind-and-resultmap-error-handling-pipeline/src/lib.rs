//! Result.bind and Result.map — Error Handling Pipeline
//!
//! Chain computations that may fail using Result combinators.
//! Mirrors OCaml's `Result.bind` and `Result.map` pipeline style.

/// Parse a string into an integer, returning an error string on failure.
///
/// OCaml: `int_of_string_opt` wrapped in `Result`
pub fn parse_int(s: &str) -> Result<i64, String> {
    s.parse::<i64>().map_err(|_| format!("Not a number: {s}"))
}

/// Require n > 0.
pub fn check_positive(n: i64) -> Result<i64, String> {
    if n > 0 {
        Ok(n)
    } else {
        Err("Must be positive".to_string())
    }
}

/// Require n <= 100.
pub fn check_range(n: i64) -> Result<i64, String> {
    if n <= 100 {
        Ok(n)
    } else {
        Err("Must be <= 100".to_string())
    }
}

/// Idiomatic Rust: `and_then` chains (= OCaml `Result.bind`), `map` transforms.
///
/// Pipeline: parse → check_positive → check_range → double
pub fn validate(s: &str) -> Result<i64, String> {
    parse_int(s)
        .and_then(check_positive)
        .and_then(check_range)
        .map(|n| n * 2)
}

/// Functional style using the `?` operator — short-circuits on the first error.
/// Cleaner when there are many steps; mirrors OCaml monadic `let*` notation.
pub fn validate_question_mark(s: &str) -> Result<i64, String> {
    let n = parse_int(s)?;
    let n = check_positive(n)?;
    let n = check_range(n)?;
    Ok(n * 2)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- parse_int ---

    #[test]
    fn parse_valid_integer() {
        assert_eq!(parse_int("42"), Ok(42));
    }

    #[test]
    fn parse_negative_integer() {
        assert_eq!(parse_int("-7"), Ok(-7));
    }

    #[test]
    fn parse_non_numeric_returns_error() {
        assert_eq!(parse_int("abc"), Err("Not a number: abc".to_string()));
    }

    #[test]
    fn parse_empty_string_returns_error() {
        assert!(parse_int("").is_err());
    }

    // --- check_positive ---

    #[test]
    fn positive_number_passes() {
        assert_eq!(check_positive(1), Ok(1));
    }

    #[test]
    fn zero_fails_positive_check() {
        assert_eq!(check_positive(0), Err("Must be positive".to_string()));
    }

    #[test]
    fn negative_fails_positive_check() {
        assert_eq!(check_positive(-5), Err("Must be positive".to_string()));
    }

    // --- check_range ---

    #[test]
    fn value_at_boundary_passes_range() {
        assert_eq!(check_range(100), Ok(100));
    }

    #[test]
    fn value_above_100_fails_range() {
        assert_eq!(check_range(101), Err("Must be <= 100".to_string()));
    }

    // --- validate (and_then chain) ---

    #[test]
    fn validate_valid_input_doubles() {
        assert_eq!(validate("42"), Ok(84));
    }

    #[test]
    fn validate_boundary_value() {
        assert_eq!(validate("100"), Ok(200));
    }

    #[test]
    fn validate_non_numeric_short_circuits() {
        assert_eq!(validate("abc"), Err("Not a number: abc".to_string()));
    }

    #[test]
    fn validate_zero_fails_positive() {
        assert_eq!(validate("0"), Err("Must be positive".to_string()));
    }

    #[test]
    fn validate_negative_fails_positive() {
        assert_eq!(validate("-1"), Err("Must be positive".to_string()));
    }

    #[test]
    fn validate_too_large_fails_range() {
        assert_eq!(validate("101"), Err("Must be <= 100".to_string()));
    }

    // --- validate_question_mark (? operator) ---

    #[test]
    fn question_mark_gives_same_result_for_valid() {
        assert_eq!(validate_question_mark("42"), Ok(84));
    }

    #[test]
    fn question_mark_short_circuits_on_parse_error() {
        assert!(validate_question_mark("xyz").is_err());
    }

    #[test]
    fn question_mark_short_circuits_on_range_error() {
        assert_eq!(
            validate_question_mark("200"),
            Err("Must be <= 100".to_string())
        );
    }

    #[test]
    fn both_implementations_agree() {
        let cases = ["42", "0", "-5", "100", "101", "hello", "1"];
        for s in cases {
            assert_eq!(
                validate(s),
                validate_question_mark(s),
                "mismatch for input {s:?}"
            );
        }
    }
}

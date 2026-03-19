// Example 056: Result Monad
// Result monad: chain computations that may fail with error info

// Approach 1: and_then chains
fn parse_int(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|_| format!("Not an integer: {}", s))
}

fn check_positive(n: i32) -> Result<i32, String> {
    if n > 0 {
        Ok(n)
    } else {
        Err(format!("Not positive: {}", n))
    }
}

fn check_even(n: i32) -> Result<i32, String> {
    if n % 2 == 0 {
        Ok(n)
    } else {
        Err(format!("Not even: {}", n))
    }
}

fn validate_input(s: &str) -> Result<i32, String> {
    parse_int(s).and_then(check_positive).and_then(check_even)
}

// Approach 2: Using ? operator (Rust's monadic do-notation)
fn validate_input_question(s: &str) -> Result<i32, String> {
    let n = parse_int(s)?;
    let n = check_positive(n)?;
    let n = check_even(n)?;
    Ok(n)
}

// Approach 3: Map and bind combined
fn double_validated(s: &str) -> Result<i32, String> {
    validate_input(s).map(|n| n * 2)
}

// Bonus: custom error type with From for automatic ? conversion
#[derive(Debug, PartialEq)]
enum ValidationError {
    ParseError(String),
    NotPositive(i32),
    NotEven(i32),
}

fn parse_int_typed(s: &str) -> Result<i32, ValidationError> {
    s.parse::<i32>()
        .map_err(|_| ValidationError::ParseError(s.to_string()))
}

fn check_positive_typed(n: i32) -> Result<i32, ValidationError> {
    if n > 0 {
        Ok(n)
    } else {
        Err(ValidationError::NotPositive(n))
    }
}

fn check_even_typed(n: i32) -> Result<i32, ValidationError> {
    if n % 2 == 0 {
        Ok(n)
    } else {
        Err(ValidationError::NotEven(n))
    }
}

fn validate_typed(s: &str) -> Result<i32, ValidationError> {
    let n = parse_int_typed(s)?;
    let n = check_positive_typed(n)?;
    check_even_typed(n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_input() {
        assert_eq!(validate_input("42"), Ok(42));
    }

    #[test]
    fn test_parse_error() {
        assert_eq!(validate_input("hello"), Err("Not an integer: hello".into()));
    }

    #[test]
    fn test_not_positive() {
        assert_eq!(validate_input("-4"), Err("Not positive: -4".into()));
    }

    #[test]
    fn test_not_even() {
        assert_eq!(validate_input("7"), Err("Not even: 7".into()));
    }

    #[test]
    fn test_question_mark_same_as_and_then() {
        for s in &["42", "hello", "-4", "7"] {
            assert_eq!(validate_input(s), validate_input_question(s));
        }
    }

    #[test]
    fn test_double() {
        assert_eq!(double_validated("42"), Ok(84));
    }

    #[test]
    fn test_typed_errors() {
        assert_eq!(validate_typed("42"), Ok(42));
        assert_eq!(
            validate_typed("bad"),
            Err(ValidationError::ParseError("bad".into()))
        );
        assert_eq!(validate_typed("-2"), Err(ValidationError::NotPositive(-2)));
        assert_eq!(validate_typed("3"), Err(ValidationError::NotEven(3)));
    }
}

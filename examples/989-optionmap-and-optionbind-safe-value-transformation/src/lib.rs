// Option.map and Option.bind — Safe Value Transformation
// Demonstrates chaining operations on optional values using
// `Option::map` and `Option::and_then` (OCaml's `Option.bind`).

// Solution 1: Idiomatic Rust — method chaining on Option
// `Option::and_then` is the Rust equivalent of OCaml's `Option.bind`
pub fn parse_int(s: &str) -> Option<i32> {
    s.parse().ok()
}

pub fn safe_div(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

/// Chain: parse → double → divide — idiomatic method chaining
pub fn parse_double_divide(s: &str, divisor: i32) -> Option<i32> {
    parse_int(s)
        .map(|x| x * 2)
        .and_then(|x| safe_div(x, divisor))
}

// Solution 2: Explicit early-return pattern matching — mirrors OCaml exhaustive match style
// Uses `return` to short-circuit on None, showing the control flow explicitly
pub fn parse_double_divide_explicit(s: &str, divisor: i32) -> Option<i32> {
    let n = match s.parse::<i32>() {
        Ok(n) => n,
        Err(_) => return None,
    };
    let doubled = n * 2;
    match divisor {
        0 => None,
        d => Some(doubled / d),
    }
}

// Solution 3: Using the `?` operator — idiomatic Rust for fallible chains
// `?` desugars to: return None if the value is None
pub fn parse_double_divide_question(s: &str, divisor: i32) -> Option<i32> {
    let n: i32 = s.parse().ok()?;
    let doubled = n * 2;
    safe_div(doubled, divisor)
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- parse_int ---

    #[test]
    fn test_parse_int_valid() {
        assert_eq!(parse_int("42"), Some(42));
    }

    #[test]
    fn test_parse_int_invalid() {
        assert_eq!(parse_int("abc"), None);
    }

    #[test]
    fn test_parse_int_empty() {
        assert_eq!(parse_int(""), None);
    }

    // --- safe_div ---

    #[test]
    fn test_safe_div_normal() {
        assert_eq!(safe_div(84, 7), Some(12));
    }

    #[test]
    fn test_safe_div_by_zero() {
        assert_eq!(safe_div(84, 0), None);
    }

    // --- parse_double_divide (idiomatic) ---

    #[test]
    fn test_chain_happy_path() {
        // "42" -> 42 -> 84 -> 84/7 = 12
        assert_eq!(parse_double_divide("42", 7), Some(12));
    }

    #[test]
    fn test_chain_parse_fails() {
        assert_eq!(parse_double_divide("not-a-number", 7), None);
    }

    #[test]
    fn test_chain_divide_by_zero() {
        assert_eq!(parse_double_divide("42", 0), None);
    }

    #[test]
    fn test_chain_negative() {
        // "-10" -> -10 -> -20 -> -20/4 = -5
        assert_eq!(parse_double_divide("-10", 4), Some(-5));
    }

    // --- parse_double_divide_explicit ---

    #[test]
    fn test_explicit_happy_path() {
        assert_eq!(parse_double_divide_explicit("42", 7), Some(12));
    }

    #[test]
    fn test_explicit_parse_fails() {
        assert_eq!(parse_double_divide_explicit("bad", 7), None);
    }

    #[test]
    fn test_explicit_divide_by_zero() {
        assert_eq!(parse_double_divide_explicit("42", 0), None);
    }

    // --- parse_double_divide_question ---

    #[test]
    fn test_question_happy_path() {
        assert_eq!(parse_double_divide_question("42", 7), Some(12));
    }

    #[test]
    fn test_question_parse_fails() {
        assert_eq!(parse_double_divide_question("oops", 7), None);
    }

    #[test]
    fn test_question_divide_by_zero() {
        assert_eq!(parse_double_divide_question("42", 0), None);
    }
}

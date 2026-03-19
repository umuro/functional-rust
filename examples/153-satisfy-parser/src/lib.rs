// Example 153: Satisfy Parser
// Parse a character matching a predicate

type ParseResult<'a, T> = Result<(T, &'a str), String>;
type Parser<'a, T> = Box<dyn Fn(&'a str) -> ParseResult<'a, T> + 'a>;

// ============================================================
// Approach 1: satisfy with predicate and description
// ============================================================

fn satisfy<'a, F>(pred: F, desc: &str) -> Parser<'a, char>
where
    F: Fn(char) -> bool + 'a,
{
    let desc = desc.to_string();
    Box::new(move |input: &'a str| match input.chars().next() {
        Some(c) if pred(c) => Ok((c, &input[c.len_utf8()..])),
        Some(c) => Err(format!("'{}' does not satisfy {}", c, desc)),
        None => Err(format!("Expected {}, got EOF", desc)),
    })
}

// ============================================================
// Approach 2: Build specific parsers from satisfy
// ============================================================

fn is_digit<'a>() -> Parser<'a, char> {
    satisfy(|c| c.is_ascii_digit(), "digit")
}

fn is_letter<'a>() -> Parser<'a, char> {
    satisfy(|c| c.is_ascii_alphabetic(), "letter")
}

fn is_alphanumeric<'a>() -> Parser<'a, char> {
    satisfy(|c| c.is_ascii_alphanumeric(), "alphanumeric")
}

fn is_whitespace_char<'a>() -> Parser<'a, char> {
    satisfy(|c| c.is_ascii_whitespace(), "whitespace")
}

fn is_uppercase<'a>() -> Parser<'a, char> {
    satisfy(|c| c.is_ascii_uppercase(), "uppercase letter")
}

fn is_lowercase<'a>() -> Parser<'a, char> {
    satisfy(|c| c.is_ascii_lowercase(), "lowercase letter")
}

// ============================================================
// Approach 3: satisfy_or with custom error function
// ============================================================

fn satisfy_or<'a, F, E>(pred: F, on_fail: E) -> Parser<'a, char>
where
    F: Fn(char) -> bool + 'a,
    E: Fn(char) -> String + 'a,
{
    Box::new(move |input: &'a str| match input.chars().next() {
        Some(c) if pred(c) => Ok((c, &input[c.len_utf8()..])),
        Some(c) => Err(on_fail(c)),
        None => Err("Unexpected EOF".to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_digit_success() {
        let p = is_digit();
        assert_eq!(p("42"), Ok(('4', "2")));
    }

    #[test]
    fn test_digit_failure() {
        let p = is_digit();
        assert!(p("abc").is_err());
    }

    #[test]
    fn test_letter_success() {
        let p = is_letter();
        assert_eq!(p("hello"), Ok(('h', "ello")));
    }

    #[test]
    fn test_letter_failure() {
        let p = is_letter();
        assert!(p("123").is_err());
    }

    #[test]
    fn test_alphanumeric() {
        let p = is_alphanumeric();
        assert_eq!(p("a1"), Ok(('a', "1")));
        assert_eq!(p("1a"), Ok(('1', "a")));
        assert!(p("!x").is_err());
    }

    #[test]
    fn test_whitespace() {
        let p = is_whitespace_char();
        assert_eq!(p(" x"), Ok((' ', "x")));
        assert_eq!(p("\tx"), Ok(('\t', "x")));
        assert!(p("x").is_err());
    }

    #[test]
    fn test_uppercase() {
        let p = is_uppercase();
        assert_eq!(p("Hello"), Ok(('H', "ello")));
        assert!(p("hello").is_err());
    }

    #[test]
    fn test_lowercase() {
        let p = is_lowercase();
        assert_eq!(p("hello"), Ok(('h', "ello")));
        assert!(p("Hello").is_err());
    }

    #[test]
    fn test_custom_predicate() {
        let hex = satisfy(|c| c.is_ascii_hexdigit(), "hex digit");
        assert_eq!(hex("ff"), Ok(('f', "f")));
        assert!(hex("zz").is_err());
    }

    #[test]
    fn test_satisfy_or_custom_error() {
        let p = satisfy_or(|c| c == '@', |c| format!("Expected '@', found '{}'", c));
        assert_eq!(p("@hello"), Ok(('@', "hello")));
        assert_eq!(p("hello"), Err("Expected '@', found 'h'".to_string()));
    }

    #[test]
    fn test_empty_input() {
        let p = is_digit();
        assert!(p("").is_err());
    }
}

// Example 154: String Parser
// Parse exact string literals

type ParseResult<'a, T> = Result<(T, &'a str), String>;
type Parser<'a, T> = Box<dyn Fn(&'a str) -> ParseResult<'a, T> + 'a>;

// ============================================================
// Approach 1: Parse exact string (tag)
// ============================================================

fn tag<'a>(expected: &str) -> Parser<'a, &'a str> {
    let expected_owned = expected.to_string();
    Box::new(move |input: &'a str| {
        if input.starts_with(&expected_owned) {
            let rest = &input[expected_owned.len()..];
            Ok((&input[..expected_owned.len()], rest))
        } else {
            Err(format!("Expected \"{}\"", expected_owned))
        }
    })
}

// ============================================================
// Approach 2: Case-insensitive string match
// ============================================================

fn tag_no_case<'a>(expected: &str) -> Parser<'a, &'a str> {
    let expected_lower = expected.to_lowercase();
    let len = expected.len();
    Box::new(move |input: &'a str| {
        if input.len() >= len && input[..len].to_lowercase() == expected_lower {
            Ok((&input[..len], &input[len..]))
        } else {
            Err(format!("Expected \"{}\" (case insensitive)", expected_lower))
        }
    })
}

// ============================================================
// Approach 3: Build from character-by-character matching
// ============================================================

fn string_from_chars<'a>(expected: &str) -> Parser<'a, String> {
    let expected = expected.to_string();
    Box::new(move |input: &'a str| {
        let mut remaining = input;
        for expected_char in expected.chars() {
            match remaining.chars().next() {
                Some(c) if c == expected_char => {
                    remaining = &remaining[c.len_utf8()..];
                }
                Some(c) => return Err(format!("Expected '{}', got '{}'", expected_char, c)),
                None => return Err(format!("Expected '{}', got EOF", expected_char)),
            }
        }
        Ok((expected.clone(), remaining))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_match() {
        let p = tag("hello");
        assert_eq!(p("hello world"), Ok(("hello", " world")));
    }

    #[test]
    fn test_tag_exact() {
        let p = tag("hello");
        assert_eq!(p("hello"), Ok(("hello", "")));
    }

    #[test]
    fn test_tag_no_match() {
        let p = tag("hello");
        assert!(p("world").is_err());
    }

    #[test]
    fn test_tag_too_short() {
        let p = tag("hello");
        assert!(p("hel").is_err());
    }

    #[test]
    fn test_tag_no_case_upper() {
        let p = tag_no_case("Hello");
        assert_eq!(p("HELLO world"), Ok(("HELLO", " world")));
    }

    #[test]
    fn test_tag_no_case_mixed() {
        let p = tag_no_case("hello");
        assert_eq!(p("HeLLo!"), Ok(("HeLLo", "!")));
    }

    #[test]
    fn test_string_from_chars() {
        let p = string_from_chars("abc");
        assert_eq!(p("abcdef"), Ok(("abc".to_string(), "def")));
    }

    #[test]
    fn test_string_from_chars_fail() {
        let p = string_from_chars("abc");
        assert!(p("axc").is_err());
    }

    #[test]
    fn test_tag_empty_string() {
        let p = tag("");
        assert_eq!(p("anything"), Ok(("", "anything")));
    }
}

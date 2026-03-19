// Example 152: Character Parsers
// Parse single characters: char_parser, any_char, none_of, one_of

type ParseResult<'a, T> = Result<(T, &'a str), String>;
type Parser<'a, T> = Box<dyn Fn(&'a str) -> ParseResult<'a, T> + 'a>;

// ============================================================
// Approach 1: Parse a specific character
// ============================================================

fn char_parser<'a>(expected: char) -> Parser<'a, char> {
    Box::new(move |input: &'a str| match input.chars().next() {
        Some(c) if c == expected => Ok((c, &input[c.len_utf8()..])),
        Some(c) => Err(format!("Expected '{}', got '{}'", expected, c)),
        None => Err(format!("Expected '{}', got EOF", expected)),
    })
}

// ============================================================
// Approach 2: Parse any character
// ============================================================

fn any_char<'a>() -> Parser<'a, char> {
    Box::new(|input: &'a str| match input.chars().next() {
        Some(c) => Ok((c, &input[c.len_utf8()..])),
        None => Err("Expected any character, got EOF".to_string()),
    })
}

// ============================================================
// Approach 3: Parse char NOT in set / IN set
// ============================================================

fn none_of<'a>(chars: Vec<char>) -> Parser<'a, char> {
    Box::new(move |input: &'a str| match input.chars().next() {
        Some(c) if !chars.contains(&c) => Ok((c, &input[c.len_utf8()..])),
        Some(c) => Err(format!("Unexpected character '{}'", c)),
        None => Err("Expected a character, got EOF".to_string()),
    })
}

fn one_of<'a>(chars: Vec<char>) -> Parser<'a, char> {
    Box::new(move |input: &'a str| match input.chars().next() {
        Some(c) if chars.contains(&c) => Ok((c, &input[c.len_utf8()..])),
        Some(c) => Err(format!("Character '{}' not in allowed set", c)),
        None => Err("Expected a character, got EOF".to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_parser_match() {
        let p = char_parser('a');
        assert_eq!(p("abc"), Ok(('a', "bc")));
    }

    #[test]
    fn test_char_parser_no_match() {
        let p = char_parser('a');
        assert!(p("xyz").is_err());
    }

    #[test]
    fn test_char_parser_empty() {
        let p = char_parser('a');
        assert!(p("").is_err());
    }

    #[test]
    fn test_any_char_success() {
        let p = any_char();
        assert_eq!(p("hello"), Ok(('h', "ello")));
    }

    #[test]
    fn test_any_char_single() {
        let p = any_char();
        assert_eq!(p("x"), Ok(('x', "")));
    }

    #[test]
    fn test_any_char_empty() {
        let p = any_char();
        assert!(p("").is_err());
    }

    #[test]
    fn test_none_of_allowed() {
        let p = none_of(vec!['x', 'y', 'z']);
        assert_eq!(p("abc"), Ok(('a', "bc")));
    }

    #[test]
    fn test_none_of_blocked() {
        let p = none_of(vec!['a', 'b']);
        assert!(p("abc").is_err());
    }

    #[test]
    fn test_one_of_match() {
        let p = one_of(vec!['a', 'b', 'c']);
        assert_eq!(p("beta"), Ok(('b', "eta")));
    }

    #[test]
    fn test_one_of_no_match() {
        let p = one_of(vec!['x', 'y']);
        assert!(p("abc").is_err());
    }

    #[test]
    fn test_unicode_char() {
        let p = char_parser('é');
        assert_eq!(p("école"), Ok(('é', "cole")));
    }
}

#![allow(clippy::all)]
// Example 151: Introduction to Parser Combinators
// Defining the core Parser type and running basic parsers

/// Core type: a parse result is either (value, remaining_input) or an error
type ParseResult<'a, T> = Result<(T, &'a str), String>;

// ============================================================
// Approach 1: Parser as a plain function
// ============================================================

fn parse_char_a(input: &str) -> ParseResult<char> {
    match input.chars().next() {
        Some('a') => Ok(('a', &input[1..])),
        Some(c) => Err(format!("Expected 'a', got '{}'", c)),
        None => Err("Expected 'a', got end of input".to_string()),
    }
}

// ============================================================
// Approach 2: Parser as a boxed closure (our standard type)
// ============================================================

type Parser<'a, T> = Box<dyn Fn(&'a str) -> ParseResult<'a, T> + 'a>;

/// Create a parser that always succeeds with the given value
fn pure<'a, T: Clone + 'a>(value: T) -> Parser<'a, T> {
    Box::new(move |input| Ok((value.clone(), input)))
}

/// Create a parser that always fails with the given message
fn fail<'a, T: 'a>(msg: &str) -> Parser<'a, T> {
    let msg = msg.to_string();
    Box::new(move |_input| Err(msg.clone()))
}

/// Create a parser that matches a specific character
fn char_p<'a>(expected: char) -> Parser<'a, char> {
    Box::new(move |input: &'a str| match input.chars().next() {
        Some(c) if c == expected => Ok((c, &input[c.len_utf8()..])),
        Some(c) => Err(format!("Expected '{}', got '{}'", expected, c)),
        None => Err(format!("Expected '{}', got end of input", expected)),
    })
}

/// Run a parser on input
fn run_parser<'a, T>(parser: &Parser<'a, T>, input: &'a str) -> ParseResult<'a, T> {
    parser(input)
}

// ============================================================
// Approach 3: Parser as a trait object (alternative design)
// ============================================================

trait Parse<T> {
    fn parse<'a>(&self, input: &'a str) -> ParseResult<'a, T>;
}

struct CharParser {
    expected: char,
}

impl Parse<char> for CharParser {
    fn parse<'a>(&self, input: &'a str) -> ParseResult<'a, char> {
        match input.chars().next() {
            Some(c) if c == self.expected => Ok((c, &input[c.len_utf8()..])),
            Some(c) => Err(format!("Expected '{}', got '{}'", self.expected, c)),
            None => Err(format!("Expected '{}', got EOF", self.expected)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plain_function_success() {
        assert_eq!(parse_char_a("abc"), Ok(('a', "bc")));
    }

    #[test]
    fn test_plain_function_failure() {
        assert!(parse_char_a("xyz").is_err());
    }

    #[test]
    fn test_plain_function_empty() {
        assert!(parse_char_a("").is_err());
    }

    #[test]
    fn test_char_parser_success() {
        let p = char_p('h');
        assert_eq!(run_parser(&p, "hello"), Ok(('h', "ello")));
    }

    #[test]
    fn test_char_parser_failure() {
        let p = char_p('h');
        assert!(run_parser(&p, "world").is_err());
    }

    #[test]
    fn test_pure() {
        let p = pure(42);
        assert_eq!(run_parser(&p, "hello"), Ok((42, "hello")));
    }

    #[test]
    fn test_fail() {
        let p: Parser<i32> = fail("oops");
        assert_eq!(run_parser(&p, "hello"), Err("oops".to_string()));
    }

    #[test]
    fn test_trait_parser() {
        let p = CharParser { expected: 'z' };
        assert_eq!(p.parse("zoo"), Ok(('z', "oo")));
    }

    #[test]
    fn test_trait_parser_failure() {
        let p = CharParser { expected: 'z' };
        assert!(p.parse("abc").is_err());
    }
}

#![allow(clippy::all)]
//! Lifetimes in Enums
//!
//! Enum variants containing references require lifetime parameters.

/// Token borrowing from the input string.
#[derive(Debug, PartialEq, Clone)]
pub enum Token<'a> {
    Word(&'a str),
    Number(i64),
    Punctuation(char),
    End,
}

/// Parse result that borrows from input.
#[derive(Debug)]
pub enum ParseResult<'a, T> {
    Ok(T, &'a str),       // value + remaining input
    Err(&'a str, String), // failing input + error message
}

/// JSON-like value that may borrow from source.
#[derive(Debug, Clone, PartialEq)]
pub enum JsonValue<'a> {
    Null,
    Bool(bool),
    Number(f64),
    String(&'a str), // borrows from source
    Array(Vec<JsonValue<'a>>),
    Object(Vec<(&'a str, JsonValue<'a>)>),
}

/// Simple tokenizer.
pub fn tokenize(input: &str) -> Vec<Token<'_>> {
    let mut tokens = Vec::new();
    let mut chars = input.char_indices().peekable();

    while let Some((i, c)) = chars.next() {
        match c {
            ' ' | '\t' | '\n' => continue,
            '.' | ',' | '!' | '?' => tokens.push(Token::Punctuation(c)),
            '0'..='9' => {
                let start = i;
                while chars
                    .peek()
                    .map(|(_, c)| c.is_ascii_digit())
                    .unwrap_or(false)
                {
                    chars.next();
                }
                let end = chars.peek().map(|(i, _)| *i).unwrap_or(input.len());
                let num: i64 = input[start..end].parse().unwrap_or(0);
                tokens.push(Token::Number(num));
            }
            'a'..='z' | 'A'..='Z' => {
                let start = i;
                while chars
                    .peek()
                    .map(|(_, c)| c.is_alphanumeric())
                    .unwrap_or(false)
                {
                    chars.next();
                }
                let end = chars.peek().map(|(i, _)| *i).unwrap_or(input.len());
                tokens.push(Token::Word(&input[start..end]));
            }
            _ => {}
        }
    }
    tokens.push(Token::End);
    tokens
}

impl<'a> JsonValue<'a> {
    pub fn is_null(&self) -> bool {
        matches!(self, JsonValue::Null)
    }

    pub fn as_str(&self) -> Option<&'a str> {
        match self {
            JsonValue::String(s) => Some(s),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let tokens = tokenize("hello world 123");
        assert_eq!(
            tokens,
            vec![
                Token::Word("hello"),
                Token::Word("world"),
                Token::Number(123),
                Token::End
            ]
        );
    }

    #[test]
    fn test_tokenize_punctuation() {
        let tokens = tokenize("hello, world!");
        assert!(tokens.contains(&Token::Punctuation(',')));
        assert!(tokens.contains(&Token::Punctuation('!')));
    }

    #[test]
    fn test_json_value() {
        let source = "hello";
        let value = JsonValue::String(source);
        assert_eq!(value.as_str(), Some("hello"));
    }

    #[test]
    fn test_json_nested() {
        let key = "name";
        let val = "John";
        let obj = JsonValue::Object(vec![(key, JsonValue::String(val))]);
        if let JsonValue::Object(pairs) = obj {
            assert_eq!(pairs[0].0, "name");
        }
    }

    #[test]
    fn test_parse_result() {
        let input = "remaining";
        let result: ParseResult<i32> = ParseResult::Ok(42, input);
        if let ParseResult::Ok(v, rest) = result {
            assert_eq!(v, 42);
            assert_eq!(rest, "remaining");
        }
    }
}

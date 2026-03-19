#![allow(clippy::all)]
//! Lifetime Elision Rules
//!
//! When and how Rust infers lifetimes automatically.

/// Rule 1: Each input ref gets own lifetime.
/// Elided: fn strlen(s: &str) -> usize
/// Expanded: fn strlen<'a>(s: &'a str) -> usize
pub fn strlen(s: &str) -> usize {
    s.len()
}

/// Rule 2: If one input lifetime, output gets it.
/// Elided: fn first_word(s: &str) -> &str
/// Expanded: fn first_word<'a>(s: &'a str) -> &'a str
pub fn first_word(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

/// Rule 3: If &self or &mut self, output gets self's lifetime.
pub struct Parser<'a> {
    input: &'a str,
}

impl<'a> Parser<'a> {
    /// Elided: fn remaining(&self) -> &str
    /// Expanded: fn remaining(&self) -> &'a str
    pub fn remaining(&self) -> &str {
        self.input
    }
}

/// Multiple inputs: cannot elide output lifetime.
/// Must be explicit: fn longer<'a>(x: &'a str, y: &'a str) -> &'a str
pub fn longer<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

/// No elision needed for non-reference returns.
pub fn count_words(s: &str, _other: &str) -> usize {
    s.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strlen() {
        assert_eq!(strlen("hello"), 5);
    }

    #[test]
    fn test_first_word() {
        assert_eq!(first_word("hello world"), "hello");
        assert_eq!(first_word("single"), "single");
    }

    #[test]
    fn test_parser_remaining() {
        let parser = Parser {
            input: "test input",
        };
        assert_eq!(parser.remaining(), "test input");
    }

    #[test]
    fn test_longer() {
        assert_eq!(longer("hi", "hello"), "hello");
    }

    #[test]
    fn test_count_words() {
        assert_eq!(count_words("a b c", "ignored"), 3);
    }
}

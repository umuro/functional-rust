//! # String Owning References — Self-Referential Patterns
//!
//! Patterns for owning data while referencing into it.

use std::pin::Pin;

/// Simple owned string with cached parse result
pub struct ParsedString {
    source: String,
    words: Vec<(usize, usize)>, // (start, end) indices into source
}

impl ParsedString {
    pub fn new(s: &str) -> Self {
        let source = s.to_string();
        let words: Vec<_> = source
            .match_indices(char::is_alphanumeric)
            .map(|(i, _)| (i, i + 1))
            .collect();

        // Actually find word boundaries
        let mut words = Vec::new();
        let mut start = None;

        for (i, c) in source.char_indices() {
            if c.is_alphanumeric() {
                if start.is_none() {
                    start = Some(i);
                }
            } else if let Some(s) = start {
                words.push((s, i));
                start = None;
            }
        }
        if let Some(s) = start {
            words.push((s, source.len()));
        }

        Self { source, words }
    }

    pub fn get_word(&self, index: usize) -> Option<&str> {
        self.words.get(index).map(|(start, end)| &self.source[*start..*end])
    }

    pub fn word_count(&self) -> usize {
        self.words.len()
    }

    pub fn source(&self) -> &str {
        &self.source
    }
}

/// Cow-based approach
use std::borrow::Cow;

pub enum StringOrStatic<'a> {
    Static(&'static str),
    Owned(String),
    Borrowed(&'a str),
}

impl<'a> StringOrStatic<'a> {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Static(s) => s,
            Self::Owned(s) => s,
            Self::Borrowed(s) => s,
        }
    }

    pub fn into_owned(self) -> String {
        match self {
            Self::Static(s) => s.to_string(),
            Self::Owned(s) => s,
            Self::Borrowed(s) => s.to_string(),
        }
    }
}

/// Using Cow for zero-copy when possible
pub fn maybe_uppercase(s: &str) -> Cow<'_, str> {
    if s.chars().all(|c| !c.is_lowercase()) {
        Cow::Borrowed(s)
    } else {
        Cow::Owned(s.to_uppercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsed_string() {
        let ps = ParsedString::new("hello world rust");
        assert_eq!(ps.word_count(), 3);
        assert_eq!(ps.get_word(0), Some("hello"));
        assert_eq!(ps.get_word(1), Some("world"));
        assert_eq!(ps.get_word(2), Some("rust"));
        assert_eq!(ps.get_word(3), None);
    }

    #[test]
    fn test_string_or_static() {
        let s = StringOrStatic::Static("hello");
        assert_eq!(s.as_str(), "hello");

        let owned = StringOrStatic::Owned(String::from("world"));
        assert_eq!(owned.as_str(), "world");
    }

    #[test]
    fn test_cow_no_alloc() {
        let s = "ALREADY UPPER";
        let result = maybe_uppercase(s);
        assert!(matches!(result, Cow::Borrowed(_)));
    }

    #[test]
    fn test_cow_with_alloc() {
        let s = "needs uppercase";
        let result = maybe_uppercase(s);
        assert!(matches!(result, Cow::Owned(_)));
        assert_eq!(&*result, "NEEDS UPPERCASE");
    }
}

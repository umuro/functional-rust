#![allow(clippy::all)]
// Example 113: String vs &str
//
// String: owned, heap-allocated, growable, mutable
// &str: borrowed slice — a pointer + length into any existing string data

// ---------------------------------------------------------------------------
// Approach 1: Idiomatic Rust — use &str in parameters, String for ownership
// ---------------------------------------------------------------------------

/// Returns the first word (before any comma) from a string slice.
/// Accepts `&str` so callers can pass a `String` reference or a literal —
/// no allocation forced on the caller.
pub fn first_word(s: &str) -> &str {
    s.split(',').next().unwrap_or(s).trim()
}

/// Counts Unicode scalar values (chars) in any string slice.
pub fn char_count(s: &str) -> usize {
    s.chars().count()
}

/// Appends a suffix and returns a new owned `String`.
/// Takes `&str` for both — works with literals or `String` borrows.
pub fn append(base: &str, suffix: &str) -> String {
    let mut result = String::with_capacity(base.len() + suffix.len());
    result.push_str(base);
    result.push_str(suffix);
    result
}

// ---------------------------------------------------------------------------
// Approach 2: Functional / builder style — manipulate owned Strings
// ---------------------------------------------------------------------------

/// Builds a greeting by owning the name, demonstrating String mutation.
pub fn greet(name: &str) -> String {
    // String::from converts &str → String (heap allocation)
    let mut greeting = String::from("Hello, ");
    greeting.push_str(name);
    greeting.push('!');
    greeting
}

/// Splits a sentence into words, returning owned Strings.
/// Shows that collecting &str views into Strings requires an explicit clone.
pub fn words(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

/// Uppercase: &str in, new String out — no in-place mutation.
pub fn to_upper(s: &str) -> String {
    s.to_uppercase()
}

// ---------------------------------------------------------------------------
// Approach 3: Subslicing — zero-copy views into string data
// ---------------------------------------------------------------------------

/// Returns the substring at `start..start+len` as a &str slice.
/// Panics if the byte indices are not on char boundaries.
pub fn substring(s: &str, start: usize, len: usize) -> &str {
    &s[start..start + len]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_word_with_comma() {
        assert_eq!(first_word("hello, world!"), "hello");
    }

    #[test]
    fn test_first_word_no_comma() {
        assert_eq!(first_word("hello"), "hello");
    }

    #[test]
    fn test_first_word_from_owned_string() {
        // Demonstrates that first_word accepts &String via auto-deref
        let owned = String::from("foo, bar");
        assert_eq!(first_word(&owned), "foo");
    }

    #[test]
    fn test_char_count_ascii() {
        assert_eq!(char_count("hello"), 5);
    }

    #[test]
    fn test_char_count_unicode() {
        // "café" is 4 chars but 5 bytes — char_count returns chars
        assert_eq!(char_count("café"), 4);
    }

    #[test]
    fn test_append() {
        assert_eq!(append("hello", " world"), "hello world");
        // Works with &String too
        let s = String::from("foo");
        assert_eq!(append(&s, "bar"), "foobar");
    }

    #[test]
    fn test_greet() {
        assert_eq!(greet("Alice"), "Hello, Alice!");
        assert_eq!(greet("World"), "Hello, World!");
    }

    #[test]
    fn test_words() {
        assert_eq!(words("one two three"), vec!["one", "two", "three"]);
        assert_eq!(words(""), Vec::<&str>::new());
    }

    #[test]
    fn test_to_upper() {
        assert_eq!(to_upper("hello world"), "HELLO WORLD");
    }

    #[test]
    fn test_substring() {
        let s = "hello, world!";
        assert_eq!(substring(s, 7, 5), "world");
        assert_eq!(substring(s, 0, 5), "hello");
    }

    #[test]
    fn test_string_literal_is_str() {
        // &'static str: baked into the binary, no heap allocation
        let literal: &str = "static text";
        let owned: String = literal.to_owned();
        // &String coerces to &str via Deref
        let back: &str = &owned;
        assert_eq!(literal, back);
    }
}

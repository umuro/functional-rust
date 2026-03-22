#![allow(dead_code)]
//! String.split_on_char — Tokenize a String
//! See example.ml for OCaml reference
//!
//! OCaml's `String.split_on_char delim s` splits a string on a single character delimiter.
//! Rust's `str::split(delim)` is the direct equivalent — both preserve empty strings between
//! consecutive delimiters.

/// Idiomatic Rust: split a string on a delimiter character, preserving empty tokens.
/// Mirrors OCaml: `String.split_on_char delim s`
pub fn split_on_char(s: &str, delim: char) -> Vec<&str> {
    s.split(delim).collect()
}

/// Split and filter out empty tokens.
/// Mirrors OCaml: `List.filter (fun s -> s <> "") (String.split_on_char delim s)`
pub fn split_nonempty(s: &str, delim: char) -> Vec<&str> {
    s.split(delim).filter(|t| !t.is_empty()).collect()
}

/// Split on whitespace, dropping empty tokens (equivalent to OCaml's `String.split_on_char ' '`
/// followed by filtering, but handles any run of whitespace in a single step).
pub fn tokenize(s: &str) -> Vec<&str> {
    s.split_whitespace().collect()
}

/// Parse a CSV record: split on commas and trim each field.
pub fn parse_csv(line: &str) -> Vec<&str> {
    line.split(',').map(str::trim).collect()
}

/// Split only on the first occurrence of `delim`.
/// Returns `(before, after)` or `None` if delimiter not found.
/// Uses `str::split_once` — the idiomatic Rust approach.
pub fn split_first_occurrence(s: &str, delim: char) -> Option<(&str, &str)> {
    s.split_once(delim)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_empty_string() {
        // Splitting an empty string on any delimiter gives one empty token.
        assert_eq!(split_on_char("", ','), vec![""]);
    }

    #[test]
    fn test_split_single_field() {
        assert_eq!(split_on_char("hello", ','), vec!["hello"]);
    }

    #[test]
    fn test_split_csv_line() {
        let fields = split_on_char("Alice,30,Engineer,Amsterdam", ',');
        assert_eq!(fields, vec!["Alice", "30", "Engineer", "Amsterdam"]);
    }

    #[test]
    fn test_split_preserves_empty_tokens() {
        // Consecutive delimiters produce an empty string between them.
        let result = split_on_char("a,,b", ',');
        assert_eq!(result, vec!["a", "", "b"]);
    }

    #[test]
    fn test_split_nonempty_removes_empty_tokens() {
        let result = split_nonempty("  hello   world  ", ' ');
        assert_eq!(result, vec!["hello", "world"]);
    }

    #[test]
    fn test_tokenize_whitespace() {
        assert_eq!(tokenize("  hello   world  "), vec!["hello", "world"]);
    }

    #[test]
    fn test_parse_csv_trims_whitespace() {
        let result = parse_csv(" Alice , 30 , Engineer ");
        assert_eq!(result, vec!["Alice", "30", "Engineer"]);
    }

    #[test]
    fn test_split_first_occurrence_found() {
        assert_eq!(
            split_first_occurrence("key=value=extra", '='),
            Some(("key", "value=extra"))
        );
    }

    #[test]
    fn test_split_first_occurrence_not_found() {
        assert_eq!(split_first_occurrence("no-delimiter-here", '='), None);
    }
}

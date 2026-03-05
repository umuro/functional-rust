//! # String.sub and String.concat — Substring and Join
//!
//! OCaml's `String.sub s pos len` extracts a substring by position and length.
//! OCaml's `String.concat sep parts` joins a list of strings with a separator.
//!
//! Rust uses byte-index slicing (`&s[start..end]`) and `.join(sep)` on slices.

/// Idiomatic Rust: extract a substring by byte position and length.
///
/// Returns `None` if the range is out of bounds or not on a char boundary.
/// This mirrors OCaml's `String.sub` which raises `Invalid_argument` on bad ranges.
pub fn substring(s: &str, pos: usize, len: usize) -> Option<&str> {
    s.get(pos..pos + len)
}

/// Functional style: same operation built from char iterators.
///
/// Skips `pos` chars then takes `len` chars — works on Unicode char boundaries
/// rather than raw bytes. Use when you need char-level (not byte-level) semantics.
pub fn substring_chars(s: &str, pos: usize, len: usize) -> String {
    s.chars().skip(pos).take(len).collect()
}

/// Idiomatic Rust: join an iterable of string slices with a separator.
///
/// Mirrors OCaml's `String.concat sep parts`.
pub fn join(sep: &str, parts: &[&str]) -> String {
    parts.join(sep)
}

/// Functional style: join using an iterator fold, making the reduction explicit.
///
/// Shows the same structure as OCaml's `List.fold_left` behind `String.concat`.
pub fn join_iter(sep: &str, parts: &[&str]) -> String {
    let mut iter = parts.iter().copied();
    match iter.next() {
        None => String::new(),
        Some(first) => iter.fold(first.to_owned(), |mut acc, part| {
            acc.push_str(sep);
            acc.push_str(part);
            acc
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- substring ---

    #[test]
    fn test_substring_start() {
        assert_eq!(substring("Hello, World!", 0, 5), Some("Hello"));
    }

    #[test]
    fn test_substring_middle() {
        assert_eq!(substring("Hello, World!", 7, 5), Some("World"));
    }

    #[test]
    fn test_substring_empty_len() {
        assert_eq!(substring("Hello", 2, 0), Some(""));
    }

    #[test]
    fn test_substring_out_of_bounds() {
        assert_eq!(substring("Hi", 0, 10), None);
    }

    #[test]
    fn test_substring_full_string() {
        assert_eq!(substring("Rust", 0, 4), Some("Rust"));
    }

    // --- substring_chars ---

    #[test]
    fn test_substring_chars_ascii() {
        assert_eq!(substring_chars("Hello, World!", 0, 5), "Hello");
    }

    #[test]
    fn test_substring_chars_unicode() {
        // "café" — 'é' is multi-byte; char indexing is safe here
        assert_eq!(substring_chars("café", 0, 3), "caf");
        assert_eq!(substring_chars("café", 3, 1), "é");
    }

    #[test]
    fn test_substring_chars_empty_result() {
        assert_eq!(substring_chars("hello", 2, 0), "");
    }

    #[test]
    fn test_substring_chars_past_end_saturates() {
        // take saturates at end of string
        assert_eq!(substring_chars("hi", 0, 100), "hi");
    }

    // --- join ---

    #[test]
    fn test_join_pipe_separator() {
        assert_eq!(join(" | ", &["one", "two", "three"]), "one | two | three");
    }

    #[test]
    fn test_join_empty_separator() {
        assert_eq!(join("", &["a", "b", "c"]), "abc");
    }

    #[test]
    fn test_join_single_element() {
        assert_eq!(join(", ", &["only"]), "only");
    }

    #[test]
    fn test_join_empty_parts() {
        assert_eq!(join(", ", &[]), "");
    }

    // --- join_iter ---

    #[test]
    fn test_join_iter_matches_join() {
        let parts = &["one", "two", "three"];
        assert_eq!(join_iter(" | ", parts), join(" | ", parts));
    }

    #[test]
    fn test_join_iter_empty_parts() {
        assert_eq!(join_iter(", ", &[]), "");
    }

    #[test]
    fn test_join_iter_single_element() {
        assert_eq!(join_iter("-", &["solo"]), "solo");
    }

    #[test]
    fn test_join_iter_multichar_separator() {
        assert_eq!(join_iter(" -> ", &["a", "b", "c"]), "a -> b -> c");
    }
}
